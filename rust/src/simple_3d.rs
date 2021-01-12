use super::utils::*;
use nalgebra_glm as na;
use std::collections::HashMap;
use std::vec::Vec;
use wasm_bindgen::JsCast;
use web_sys::WebGlRenderingContext as GL;
use web_sys::{HtmlCanvasElement, WebGlBuffer, WebGlProgram, WebGlTexture, WebGlUniformLocation};

pub struct Simple3D {
  program: WebGlProgram,
  index_buffer: WebGlBuffer,
  num_indices: i32,
  attributes: HashMap<String, Attribute>,
  uniform_locations: HashMap<String, WebGlUniformLocation>,
  //
  diffuse_texture: WebGlTexture,
  specular_texture: WebGlTexture,
  normal_map: WebGlTexture,
  //
  diffuse_texture_ore: WebGlTexture,
  specular_texture_ore: WebGlTexture,
  normal_map_ore: WebGlTexture,
}

impl Simple3D {
  pub fn new(gl: &GL) -> Result<Simple3D, String> {
    let program = link_program(
      &gl,
      include_str!("./shaders/simple_3d.vert"),
      include_str!("./shaders/simple_3d.frag"),
    )?;

    let attributes = get_attributes(&gl, &program);
    let uniform_locations = get_uniform_locations(&gl, &program);

    buffer_attribute_data(&gl, &attributes, "a_position", &cube_positions());
    buffer_attribute_data(&gl, &attributes, "a_texcoords", &cube_texcoords());
    buffer_attribute_data(&gl, &attributes, "a_normal", &cube_normals());
    let (index_buffer, num_indices) = buffer_index_data(&gl, &cube_indices());

    // TODO: clean this up
    // tangets and bitangents --------------------
    let indices = cube_indices();
    let positions = cube_positions();
    let texcoords = cube_texcoords();

    let mut tangents: Vec<f32> = Vec::with_capacity(positions.len() as usize);
    tangents.resize(positions.len(), 0.);

    let mut bitangents: Vec<f32> = Vec::with_capacity(positions.len() as usize);
    bitangents.resize(positions.len(), 0.);

    for i in (0..indices.len()).step_by(3) {
      let v0 = na::vec3(
        positions[indices[i + 0] as usize * 3 + 0],
        positions[indices[i + 0] as usize * 3 + 1],
        positions[indices[i + 0] as usize * 3 + 2],
      );
      let v1 = na::vec3(
        positions[indices[i + 1] as usize * 3 + 0],
        positions[indices[i + 1] as usize * 3 + 1],
        positions[indices[i + 1] as usize * 3 + 2],
      );
      let v2 = na::vec3(
        positions[indices[i + 2] as usize * 3 + 0],
        positions[indices[i + 2] as usize * 3 + 1],
        positions[indices[i + 2] as usize * 3 + 2],
      );

      let uv0 = na::vec2(
        texcoords[indices[i + 0] as usize * 2 + 0],
        texcoords[indices[i + 0] as usize * 2 + 1],
      );
      let uv1 = na::vec2(
        texcoords[indices[i + 1] as usize * 2 + 0],
        texcoords[indices[i + 1] as usize * 2 + 1],
      );
      let uv2 = na::vec2(
        texcoords[indices[i + 2] as usize * 2 + 0],
        texcoords[indices[i + 2] as usize * 2 + 1],
      );

      let delta_pos_1 = v1 - v0;
      let delta_pos_2 = v2 - v0;

      let delta_uv_1 = uv1 - uv0;
      let delta_uv_2 = uv2 - uv0;

      let r = 1.0 / (delta_uv_1.x * delta_uv_2.y - delta_uv_1.y * delta_uv_2.x);
      let tangent = (delta_pos_1 * delta_uv_2.y - delta_pos_2 * delta_uv_1.y) * r;
      let bitangent = (delta_pos_2 * delta_uv_1.x - delta_pos_1 * delta_uv_2.x) * r;

      tangents[indices[i + 0] as usize * 3 + 0] = tangent.x;
      tangents[indices[i + 0] as usize * 3 + 1] = tangent.y;
      tangents[indices[i + 0] as usize * 3 + 2] = tangent.z;
      tangents[indices[i + 1] as usize * 3 + 0] = tangent.x;
      tangents[indices[i + 1] as usize * 3 + 1] = tangent.y;
      tangents[indices[i + 1] as usize * 3 + 2] = tangent.z;
      tangents[indices[i + 2] as usize * 3 + 0] = tangent.x;
      tangents[indices[i + 2] as usize * 3 + 1] = tangent.y;
      tangents[indices[i + 2] as usize * 3 + 2] = tangent.z;

      bitangents[indices[i + 0] as usize * 3 + 0] = bitangent.x;
      bitangents[indices[i + 0] as usize * 3 + 1] = bitangent.y;
      bitangents[indices[i + 0] as usize * 3 + 2] = bitangent.z;
      bitangents[indices[i + 1] as usize * 3 + 0] = bitangent.x;
      bitangents[indices[i + 1] as usize * 3 + 1] = bitangent.y;
      bitangents[indices[i + 1] as usize * 3 + 2] = bitangent.z;
      bitangents[indices[i + 2] as usize * 3 + 0] = bitangent.x;
      bitangents[indices[i + 2] as usize * 3 + 1] = bitangent.y;
      bitangents[indices[i + 2] as usize * 3 + 2] = bitangent.z;
    }

    buffer_attribute_data(&gl, &attributes, "a_tangent", tangents.as_slice());
    buffer_attribute_data(&gl, &attributes, "a_bitangent", bitangents.as_slice());
    // ----------------------------

    let diffuse_texture = load_texture(&gl, "redstone_block.png");
    let specular_texture = load_texture(&gl, "redstone_block_s.png");
    let normal_map = load_texture(&gl, "redstone_block_n.png");

    let diffuse_texture_ore = load_texture(&gl, "nether_gold_ore.png");
    let specular_texture_ore = load_texture(&gl, "nether_gold_ore_s.png");
    let normal_map_ore = load_texture(&gl, "nether_gold_ore_n.png");

    Ok(Simple3D {
      program,
      index_buffer,
      num_indices,
      attributes,
      uniform_locations,
      //
      diffuse_texture,
      specular_texture,
      normal_map,
      //
      diffuse_texture_ore,
      specular_texture_ore,
      normal_map_ore,
    })
  }

  pub fn render(&mut self, gl: &GL) {
    gl.use_program(Some(&self.program));

    set_attrib_pointers(&gl, &self.attributes);

    gl.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, Some(&self.index_buffer));

    // Redstone block ----------

    gl.active_texture(GL::TEXTURE0);
    gl.bind_texture(GL::TEXTURE_2D, Some(&self.diffuse_texture));
    gl.active_texture(GL::TEXTURE1);
    gl.bind_texture(GL::TEXTURE_2D, Some(&self.specular_texture));
    gl.active_texture(GL::TEXTURE2);
    gl.bind_texture(GL::TEXTURE_2D, Some(&self.normal_map));

    self.load_uniforms(gl);

    gl.draw_elements_with_i32(GL::TRIANGLES, self.num_indices, GL::UNSIGNED_SHORT, 0);

    // Nether gold ore -------------

    gl.active_texture(GL::TEXTURE0);
    gl.bind_texture(GL::TEXTURE_2D, Some(&self.diffuse_texture_ore));
    gl.active_texture(GL::TEXTURE1);
    gl.bind_texture(GL::TEXTURE_2D, Some(&self.specular_texture_ore));
    gl.active_texture(GL::TEXTURE2);
    gl.bind_texture(GL::TEXTURE_2D, Some(&self.normal_map_ore));

    // Position the nether gold ore differently
    let time = web_sys::window().unwrap().performance().unwrap().now() as f32;
    let scale = na::scaling(&na::vec3(100., 100., 100.));
    let rotation = na::rotation(0., &na::vec3(1., 0., 0.))
      * na::rotation(
        std::f32::consts::PI * 2. * -(time % 10000. / 10000.),
        &na::vec3(0., 1., 0.),
      )
      * na::rotation(std::f32::consts::PI * 0., &na::vec3(0., 0., 1.));
    let translation = na::translation(&na::vec3(-200., 0., -700.));
    let world = translation * rotation * scale;
    gl.uniform_matrix4fv_with_f32_array(
      Some(&self.uniform_locations.get("u_world").unwrap()),
      false,
      &world.as_slice(),
    );

    // Don't forget this boi!
    let world_inverse_transpose = na::transpose(&na::inverse(&world));
    gl.uniform_matrix4fv_with_f32_array(
      Some(
        &self
          .uniform_locations
          .get("u_world_inverse_transpose")
          .unwrap(),
      ),
      false,
      &world_inverse_transpose.as_slice(),
    );

    gl.draw_elements_with_i32(GL::TRIANGLES, self.num_indices, GL::UNSIGNED_SHORT, 0);
  }

  fn load_uniforms(&self, gl: &GL) {
    let time = web_sys::window().unwrap().performance().unwrap().now() as f32;

    // World Uniform (rot * scale * origin = model, then trans * model = world)
    let origin_shift = na::translation(&na::vec3(0., 0., 0.));
    let scale = na::scaling(&na::vec3(100., 100., 100.));
    let rotation = na::rotation(0., &na::vec3(1., 0., 0.))
      * na::rotation(
        std::f32::consts::PI * 2. * (time % 10000. / 10000.),
        &na::vec3(0., 1., 0.),
      )
      * na::rotation(std::f32::consts::PI * 0., &na::vec3(0., 0., 1.));
    let translation = na::translation(&na::vec3(200., 0., -700.));
    let world = translation * rotation * scale * origin_shift;
    gl.uniform_matrix4fv_with_f32_array(
      Some(&self.uniform_locations.get("u_world").unwrap()),
      false,
      &world.as_slice(),
    );

    // View Uniform
    let camera_rotation = na::rotation(0., &na::vec3(0., 1., 0.));
    let camera_position = na::vec3(0., 0., 0.);
    let camera_translation = na::translation(&camera_position);
    let view = na::inverse(&(camera_translation * camera_rotation));
    gl.uniform_matrix4fv_with_f32_array(
      Some(&self.uniform_locations.get("u_view").unwrap()),
      false,
      &view.as_slice(),
    );

    // Projection Uniform
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

    // World Inverse Transpose Uniform
    let world_inverse_transpose = na::transpose(&na::inverse(&world));
    gl.uniform_matrix4fv_with_f32_array(
      Some(
        &self
          .uniform_locations
          .get("u_world_inverse_transpose")
          .unwrap(),
      ),
      false,
      &world_inverse_transpose.as_slice(),
    );

    // Camera World Position Uniform
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

    // Texture + Normal Map
    gl.uniform1i(
      Some(&self.uniform_locations.get("u_diffuse_texture").unwrap()),
      0,
    );
    gl.uniform1i(
      Some(&self.uniform_locations.get("u_specular_texture").unwrap()),
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
