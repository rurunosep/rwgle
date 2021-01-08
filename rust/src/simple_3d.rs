use super::utils::link_program;
use nalgebra_glm as na;
use wasm_bindgen::JsCast;
use web_sys::WebGlRenderingContext as GL;
use web_sys::{HtmlCanvasElement, WebGlBuffer, WebGlProgram, WebGlUniformLocation};

// TODO: holy shit, figure out a way to organize and automate all these fucking uniforms and shit

pub struct Simple3D {
  program: WebGlProgram,
  num_indices: i32,
  position_buffer: WebGlBuffer,
  normal_buffer: WebGlBuffer,
  index_buffer: WebGlBuffer,
  world_location: WebGlUniformLocation,
  view_location: WebGlUniformLocation,
  projection_location: WebGlUniformLocation,
  world_inverse_transpose_location: WebGlUniformLocation,
  camera_world_position_location: WebGlUniformLocation,
  reverse_light_direction_location: WebGlUniformLocation,
}

impl Simple3D {
  pub fn new(gl: &GL) -> Result<Simple3D, String> {
    let program = link_program(
      &gl,
      include_str!("./shaders/simple_3d.vert"),
      include_str!("./shaders/simple_3d.frag"),
    )?;

    let world_location = gl.get_uniform_location(&program, "u_world").unwrap();
    let view_location = gl.get_uniform_location(&program, "u_view").unwrap();
    let projection_location = gl.get_uniform_location(&program, "u_projection").unwrap();
    let world_inverse_transpose_location = gl
      .get_uniform_location(&program, "u_world_inverse_transpose")
      .unwrap();
    let camera_world_position_location = gl
      .get_uniform_location(&program, "u_camera_world_position")
      .unwrap();
    let reverse_light_direction_location = gl
      .get_uniform_location(&program, "u_reverse_light_direction")
      .unwrap();

    let positions = cube_positions();
    let position_buffer = gl.create_buffer().unwrap();
    gl.bind_buffer(GL::ARRAY_BUFFER, Some(&position_buffer));
    unsafe {
      let array = js_sys::Float32Array::view(&positions);
      gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &array, GL::STATIC_DRAW);
    }

    let normals = cube_normals();
    let normal_buffer = gl.create_buffer().unwrap();
    gl.bind_buffer(GL::ARRAY_BUFFER, Some(&normal_buffer));
    unsafe {
      let array = js_sys::Float32Array::view(&normals);
      gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &array, GL::STATIC_DRAW);
    }

    let indices = cube_indices();
    let index_buffer = gl.create_buffer().unwrap();
    gl.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, Some(&index_buffer));
    unsafe {
      let array = js_sys::Uint16Array::view(&indices);
      gl.buffer_data_with_array_buffer_view(GL::ELEMENT_ARRAY_BUFFER, &array, GL::STATIC_DRAW);
    }

    let num_indices = indices.len() as i32;

    Ok(Simple3D {
      program,
      num_indices,
      position_buffer,
      normal_buffer,
      index_buffer,
      world_location,
      view_location,
      projection_location,
      world_inverse_transpose_location,
      camera_world_position_location,
      reverse_light_direction_location,
    })
  }

  pub fn render(&self, gl: &GL) {
    let time = web_sys::window().unwrap().performance().unwrap().now() as f32;
    gl.use_program(Some(&self.program));

    gl.bind_buffer(GL::ARRAY_BUFFER, Some(&self.position_buffer));
    gl.vertex_attrib_pointer_with_i32(0, 3, GL::FLOAT, false, 0, 0);
    gl.enable_vertex_attrib_array(0);

    gl.bind_buffer(GL::ARRAY_BUFFER, Some(&self.normal_buffer));
    gl.vertex_attrib_pointer_with_i32(1, 3, GL::FLOAT, false, 0, 0);
    gl.enable_vertex_attrib_array(1);

    gl.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, Some(&self.index_buffer));

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
    gl.uniform_matrix4fv_with_f32_array(Some(&self.world_location), false, &world.as_slice());

    let camera_rotation = na::rotation(0., &na::vec3(0., 1., 0.));
    let camera_position = na::vec3(0., 0., 0.);
    let camera_translation = na::translation(&camera_position);
    let view = na::inverse(&(camera_translation * camera_rotation));
    gl.uniform_matrix4fv_with_f32_array(Some(&self.view_location), false, &view.as_slice());

    let canvas = gl
      .canvas()
      .unwrap()
      .dyn_into::<HtmlCanvasElement>()
      .unwrap();
    let fov = 60. * (std::f32::consts::PI / 180.);
    let aspect = (canvas.width() / canvas.height()) as f32;
    let projection = na::perspective(aspect, fov, 1., 2000.);
    gl.uniform_matrix4fv_with_f32_array(
      Some(&self.projection_location),
      false,
      &projection.as_slice(),
    );

    let world_inverse_transpose = na::transpose(&na::inverse(&world));
    gl.uniform_matrix4fv_with_f32_array(
      Some(&self.world_inverse_transpose_location),
      false,
      &world_inverse_transpose.as_slice(),
    );

    gl.uniform3fv_with_f32_array(
      Some(&self.camera_world_position_location),
      camera_position.as_slice(),
    );

    let reverse_light_direction = na::normalize(&na::vec3(0.5, 0.7, 1.0));
    gl.uniform3fv_with_f32_array(
      Some(&self.reverse_light_direction_location),
      reverse_light_direction.as_slice(),
    );

    gl.draw_elements_with_i32(GL::TRIANGLES, self.num_indices, GL::UNSIGNED_SHORT, 0);
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
