use super::utils::*;
use nalgebra_glm as na;
use std::collections::HashMap;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use web_sys::WebGlRenderingContext as GL;
use web_sys::{
  HtmlCanvasElement, WebGlBuffer, WebGlProgram, WebGlTexture, WebGlUniformLocation,
  WebGlVertexArrayObject,
};

pub struct Simple3D {
  program: WebGlProgram,
  uniform_locations: HashMap<String, WebGlUniformLocation>,
  redstone: Object,
  nether_gold: Object,
}

impl Simple3D {
  pub async fn new(gl: &GL) -> Result<Simple3D, String> {
    let program = link_program(
      &gl,
      include_str!("./shaders/simple_3d.vert"),
      include_str!("./shaders/simple_3d.frag"),
    )?;

    let uniform_locations = get_uniform_locations(&gl, &program);

    let cube_model = Rc::new(Model::new(
      &gl,
      &get_attributes(&gl, &program),
      &cube_indices(),
      &cube_positions(),
      &cube_texcoords(),
      &cube_normals(),
    ));

    let redstone = Object {
      model: Rc::clone(&cube_model),
      color_map: load_texture(&gl, "redstone_block.png"),
      specular_map: load_texture(&gl, "redstone_block_s.png"),
      normal_map: load_texture(&gl, "redstone_block_n.png"),
      scale: na::scaling(&na::vec3(100., 100., 100.)),
      rotation: na::identity(),
      translation: na::translation(&na::vec3(200., 0., -700.)),
    };

    let nether_gold = Object {
      model: Rc::clone(&cube_model),
      color_map: load_texture(&gl, "nether_gold_ore.png"),
      specular_map: load_texture(&gl, "nether_gold_ore_s.png"),
      normal_map: load_texture(&gl, "nether_gold_ore_n.png"),
      scale: na::scaling(&na::vec3(100., 100., 100.)),
      rotation: na::identity(),
      translation: na::translation(&na::vec3(-200., 0., -700.)),
    };

    Ok(Simple3D {
      program,
      uniform_locations,
      redstone,
      nether_gold,
    })
  }

  pub fn render(&mut self, gl: &GL) {
    gl.use_program(Some(&self.program));
    self.load_uniforms(gl);

    let time = web_sys::window().unwrap().performance().unwrap().now() as f32;

    // Redstone
    self.redstone.rotation = na::rotation(0., &na::vec3(1., 0., 0.))
      * na::rotation(
        std::f32::consts::PI * 2. * (time % 10000. / 10000.),
        &na::vec3(0., 1., 0.),
      )
      * na::rotation(std::f32::consts::PI * 0., &na::vec3(0., 0., 1.));

    self.redstone.render(&gl, &self.uniform_locations);

    // Nether gold
    self.nether_gold.rotation = na::rotation(0., &na::vec3(1., 0., 0.))
      * na::rotation(
        std::f32::consts::PI * 2. * -(time % 10000. / 10000.),
        &na::vec3(0., 1., 0.),
      )
      * na::rotation(std::f32::consts::PI * 0., &na::vec3(0., 0., 1.));

    self.nether_gold.render(&gl, &self.uniform_locations);
  }

  fn load_uniforms(&self, gl: &GL) {
    // View
    let camera_rotation = na::rotation(0., &na::vec3(0., 1., 0.));
    let camera_position = na::vec3(0., 0., 0.);
    let camera_translation = na::translation(&camera_position);
    let view = na::inverse(&(camera_translation * camera_rotation));
    gl.uniform_matrix4fv_with_f32_array(
      Some(&self.uniform_locations.get("u_view").unwrap()),
      false,
      &view.as_slice(),
    );

    // Projection
    let canvas = gl
      .canvas()
      .unwrap()
      .dyn_into::<HtmlCanvasElement>()
      .unwrap();
    let fov = 60. * (std::f32::consts::PI / 180.);
    let aspect = (canvas.width() / canvas.height()) as f32;
    let projection = na::perspective(aspect, fov, 1., 2000.);
    gl.uniform_matrix4fv_with_f32_array(
      Some(&self.uniform_locations.get("u_projection").unwrap()),
      false,
      &projection.as_slice(),
    );

    // Camera World Position
    gl.uniform3fv_with_f32_array(
      Some(&self.uniform_locations.get("u_camera_position").unwrap()),
      camera_position.as_slice(),
    );

    // Lights
    self.make_light(&gl, 0, &[0., 0., 0.], &[1., 1., 1.], 0.001);
    //self.make_light(&gl, 1, &[0., 0., -2000.], &[1., 1., 1.], 0.001);
    //self.make_light(&gl, 2, &[800., 0., -800.], &[1., 1., 1.], 0.001);
    gl.uniform1i(
      Some(&self.uniform_locations.get("u_num_lights").unwrap()),
      1,
    );

    // Textures
    gl.uniform1i(Some(&self.uniform_locations.get("u_color_map").unwrap()), 0);
    gl.uniform1i(
      Some(&self.uniform_locations.get("u_specular_map").unwrap()),
      1,
    );
    gl.uniform1i(
      Some(&self.uniform_locations.get("u_normal_map").unwrap()),
      2,
    );
  }

  fn make_light(&self, gl: &GL, id: i32, pos: &[f32; 3], color: &[f32; 3], attentuation: f32) {
    gl.uniform3fv_with_f32_array(
      Some(
        &self
          .uniform_locations
          .get(&format!("u_lights[{}].position", id))
          .unwrap(),
      ),
      pos,
    );
    gl.uniform3fv_with_f32_array(
      Some(
        &self
          .uniform_locations
          .get(&format!("u_lights[{}].color", id))
          .unwrap(),
      ),
      color,
    );
    gl.uniform1f(
      Some(
        &self
          .uniform_locations
          .get(&format!("u_lights[{}].attentuation_coefficient", id))
          .unwrap(),
      ),
      attentuation,
    );
  }
}

struct Object {
  model: Rc<Model>,
  color_map: WebGlTexture,    // Move to model?
  specular_map: WebGlTexture, // ^
  normal_map: WebGlTexture,   // ^
  scale: na::Mat4,
  rotation: na::Mat4,
  translation: na::Mat4,
}

impl Object {
  pub fn render(&self, gl: &GL, uniform_locations: &HashMap<String, WebGlUniformLocation>) {
    // Textures
    gl.active_texture(GL::TEXTURE0);
    gl.bind_texture(GL::TEXTURE_2D, Some(&self.color_map));
    gl.active_texture(GL::TEXTURE1);
    gl.bind_texture(GL::TEXTURE_2D, Some(&self.specular_map));
    gl.active_texture(GL::TEXTURE2);
    gl.bind_texture(GL::TEXTURE_2D, Some(&self.normal_map));

    // World
    let world = self.translation * self.rotation * self.scale;
    gl.uniform_matrix4fv_with_f32_array(
      Some(uniform_locations.get("u_world").unwrap()),
      false,
      &world.as_slice(),
    );

    // World Inverse Transpose
    let world_inverse_transpose = na::transpose(&na::inverse(&world));
    gl.uniform_matrix4fv_with_f32_array(
      Some(uniform_locations.get("u_world_inverse_transpose").unwrap()),
      false,
      &world_inverse_transpose.as_slice(),
    );

    self.model.render(&gl);
  }
}

struct Model {
  vao: WebGlVertexArrayObject,
  index_buffer: WebGlBuffer,
  num_indices: i32,
}

impl Model {
  pub fn new(
    gl: &GL,
    attributes: &HashMap<String, Attribute>,
    indices: &[u16],
    positions: &[f32],
    texcoords: &[f32],
    normals: &[f32],
  ) -> Model {
    let (tangents, bitangents) = calc_tangents_bitangents(indices, positions, texcoords);

    let (index_buffer, num_indices) = buffer_index_data(&gl, indices);

    let oesvao = gl
      .get_extension("OES_vertex_array_object")
      .unwrap()
      .unwrap()
      .unchecked_into::<web_sys::OesVertexArrayObject>();

    let vao = oesvao.create_vertex_array_oes().unwrap();
    oesvao.bind_vertex_array_oes(Some(&vao));

    Self::buffer_and_set_pointer(&gl, attributes.get("a_position").unwrap(), positions);
    Self::buffer_and_set_pointer(&gl, attributes.get("a_texcoords").unwrap(), texcoords);
    Self::buffer_and_set_pointer(&gl, attributes.get("a_normal").unwrap(), normals);
    Self::buffer_and_set_pointer(
      &gl,
      attributes.get("a_tangent").unwrap(),
      tangents.as_slice(),
    );
    Self::buffer_and_set_pointer(
      &gl,
      attributes.get("a_bitangent").unwrap(),
      bitangents.as_slice(),
    );

    Model {
      vao,
      index_buffer,
      num_indices,
    }
  }

  pub fn render(&self, gl: &GL) {
    let oesvao = gl
      .get_extension("OES_vertex_array_object")
      .unwrap()
      .unwrap()
      .unchecked_into::<web_sys::OesVertexArrayObject>();
    oesvao.bind_vertex_array_oes(Some(&self.vao));

    gl.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, Some(&self.index_buffer));

    gl.draw_elements_with_i32(GL::TRIANGLES, self.num_indices, GL::UNSIGNED_SHORT, 0);
  }

  fn buffer_and_set_pointer(gl: &GL, attrib: &Attribute, data: &[f32]) {
    let buffer = gl.create_buffer().unwrap();
    gl.bind_buffer(GL::ARRAY_BUFFER, Some(&buffer));
    unsafe {
      let array = js_sys::Float32Array::view(&data);
      gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &array, GL::STATIC_DRAW);
    }
    gl.vertex_attrib_pointer_with_i32(attrib.index, attrib.size, attrib.type_, false, 0, 0);
    gl.enable_vertex_attrib_array(attrib.index);
  }
}

#[rustfmt::skip]
fn cube_positions() -> [f32; 72] {
  [
    -1.0, -1.0, 1.0,
    1.0, -1.0, 1.0,
    1.0, 1.0, 1.0,
    -1.0, 1.0, 1.0,

    -1.0, -1.0, -1.0,
    -1.0, 1.0, -1.0,
    1.0, 1.0, -1.0,
    1.0, -1.0, -1.0,

    -1.0, 1.0, -1.0,
    -1.0, 1.0, 1.0,
    1.0, 1.0, 1.0,
    1.0, 1.0, -1.0,

    -1.0, -1.0, -1.0,
    1.0, -1.0, -1.0,
    1.0, -1.0, 1.0,
    -1.0, -1.0, 1.0,

    1.0, -1.0, -1.0,
    1.0, 1.0, -1.0,
    1.0, 1.0, 1.0,
    1.0, -1.0, 1.0,

    -1.0, -1.0, -1.0,
    -1.0, -1.0, 1.0,
    -1.0, 1.0, 1.0,
    -1.0, 1.0, -1.0,
  ]
}

#[rustfmt::skip]
fn cube_normals() -> [f32; 72] {
  [
    0.0, 0.0, 1.0,
    0.0, 0.0, 1.0,
    0.0, 0.0, 1.0,
    0.0, 0.0, 1.0,

    0.0, 0.0, -1.0,
    0.0, 0.0, -1.0,
    0.0, 0.0, -1.0,
    0.0, 0.0, -1.0,

    0.0, 1.0, 0.0,
    0.0, 1.0, 0.0,
    0.0, 1.0, 0.0,
    0.0, 1.0, 0.0,

    0.0, -1.0, 0.0,
    0.0, -1.0, 0.0,
    0.0, -1.0, 0.0,
    0.0, -1.0, 0.0,

    1.0, 0.0, 0.0,
    1.0, 0.0, 0.0,
    1.0, 0.0, 0.0,
    1.0, 0.0, 0.0,

    -1.0, 0.0, 0.0,
    -1.0, 0.0, 0.0,
    -1.0, 0.0, 0.0,
    -1.0, 0.0, 0.0,
  ]
}

#[rustfmt::skip]
fn cube_indices() -> [u16; 36] {
  [
    0, 1, 2, 0, 2, 3,
    4, 5, 6, 4, 6, 7,
    8, 9, 10, 8, 10, 11,
    12, 13, 14, 12, 14,
    15, 16, 17, 18, 16, 18,
    19, 20, 21, 22, 20, 22, 23,
  ]
}

#[rustfmt::skip]
fn cube_texcoords() -> [f32; 48] {
 [
   0., 0.,
   0., 1.,
   1., 1.,
   1., 0.,

   0., 0.,
   0., 1.,
   1., 1.,
   1., 0.,

   0., 0.,
   0., 1.,
   1., 1.,
   1., 0.,

   0., 0.,
   0., 1.,
   1., 1.,
   1., 0.,

   0., 0.,
   0., 1.,
   1., 1.,
   1., 0.,

   0., 0.,
   0., 1.,
   1., 1.,
   1., 0.,
 ]
}
