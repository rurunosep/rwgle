use super::utils::*;
use nalgebra_glm as na;
use std::collections::HashMap;
use wasm_bindgen::JsCast;
use web_sys::WebGlRenderingContext as GL;
use web_sys::{HtmlCanvasElement, WebGlBuffer, WebGlProgram, WebGlUniformLocation};

pub struct Simple3D {
  program: WebGlProgram,
  index_buffer: WebGlBuffer,
  num_indices: i32,
  attributes: HashMap<String, Attribute>,
  uniform_locations: HashMap<String, WebGlUniformLocation>,
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
    buffer_attribute_data(&gl, &attributes, "a_normal", &cube_normals());

    let (index_buffer, num_indices) = buffer_index_data(&gl, &cube_indices());

    Ok(Simple3D {
      program,
      index_buffer,
      num_indices,
      attributes,
      uniform_locations,
    })
  }

  pub fn render(&self, gl: &GL) {
    gl.use_program(Some(&self.program));
    set_attrib_pointers(&gl, &self.attributes);
    gl.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, Some(&self.index_buffer));
    self.load_uniforms(gl);
    gl.draw_elements_with_i32(GL::TRIANGLES, self.num_indices, GL::UNSIGNED_SHORT, 0);
  }

  fn load_uniforms(&self, gl: &GL) {
    let time = web_sys::window().unwrap().performance().unwrap().now() as f32;

    // World Uniform
    let origin_shift = na::translation(&na::vec3(0., 0., 0.));
    let scale = na::scaling(&na::vec3(100., 100., 100.));
    let rotation = na::rotation(0., &na::vec3(1., 0., 0.))
      * na::rotation(
        std::f32::consts::PI * 2. * (time % 5000. / 5000.),
        &na::vec3(0., 1., 0.),
      )
      * na::rotation(0.5, &na::vec3(0., 0., 1.));
    let translation = na::translation(&na::vec3(0., 0., -1000.));
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
      Some(
        &self
          .uniform_locations
          .get("u_camera_world_position")
          .unwrap(),
      ),
      camera_position.as_slice(),
    );

    // Reverse Light Direction Uniform
    let reverse_light_direction = na::normalize(&na::vec3(0.7, 0.5, 1.0));
    gl.uniform3fv_with_f32_array(
      Some(
        &self
          .uniform_locations
          .get("u_reverse_light_direction")
          .unwrap(),
      ),
      reverse_light_direction.as_slice(),
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
