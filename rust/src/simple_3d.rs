use super::utils::link_program;
use nalgebra_glm as na;
use wasm_bindgen::JsCast;
use web_sys::WebGlRenderingContext as GL;
use web_sys::{HtmlCanvasElement, WebGlBuffer, WebGlProgram, WebGlUniformLocation};

pub struct Simple3D {
  program: WebGlProgram,
  num_vertices: i32,
  positions_buffer: WebGlBuffer,
  colors_buffer: WebGlBuffer,
  mvp_location: WebGlUniformLocation,
}

impl Simple3D {
  pub fn new(gl: &GL) -> Result<Simple3D, String> {
    let program = link_program(
      &gl,
      include_str!("./shaders/simple_3d.vert"),
      include_str!("./shaders/simple_3d.frag"),
    )?;

    let mvp_location = gl.get_uniform_location(&program, "u_mvp").unwrap();

    let num_vertices = 96;

    let positions = f_shape_3d_positions();
    let positions_buffer = gl.create_buffer().unwrap();
    gl.bind_buffer(GL::ARRAY_BUFFER, Some(&positions_buffer));
    unsafe {
      let array = js_sys::Float32Array::view(&positions);
      gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &array, GL::STATIC_DRAW);
    }

    let colors = f_shape_3d_colors();
    let colors_buffer = gl.create_buffer().unwrap();
    gl.bind_buffer(GL::ARRAY_BUFFER, Some(&colors_buffer));
    unsafe {
      let array = js_sys::Uint8Array::view(&colors);
      gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &array, GL::STATIC_DRAW);
    }

    Ok(Simple3D {
      program,
      num_vertices,
      positions_buffer,
      colors_buffer,
      mvp_location,
    })
  }

  pub fn render(&self, gl: &GL) {
    //let time = web_sys::window().unwrap().performance().unwrap().now() as f32;
    gl.use_program(Some(&self.program));

    gl.bind_buffer(GL::ARRAY_BUFFER, Some(&self.positions_buffer));
    gl.vertex_attrib_pointer_with_i32(0, 3, GL::FLOAT, false, 0, 0);
    gl.enable_vertex_attrib_array(0);

    gl.bind_buffer(GL::ARRAY_BUFFER, Some(&self.colors_buffer));
    gl.vertex_attrib_pointer_with_i32(1, 3, GL::UNSIGNED_BYTE, true, 0, 0);
    gl.enable_vertex_attrib_array(1);

    let canvas = gl
      .canvas()
      .unwrap()
      .dyn_into::<HtmlCanvasElement>()
      .unwrap();
    let fov = 60. * (std::f32::consts::PI / 180.);
    let aspect = (canvas.width() / canvas.height()) as f32;
    let projection = na::perspective(aspect, fov, 1., 2000.);

    let camera_rotation = na::rotation(0., &na::vec3(0., 1., 0.));
    let camera_translation = na::translation(&na::vec3(0., 0., 0.));
    let view = na::inverse(&(camera_translation * camera_rotation));

    let origin_shift = na::translation(&na::vec3(-50., -75., 0.));
    let scale = na::scaling(&na::vec3(1., 1., 1.));
    let rotation = na::rotation(std::f32::consts::PI * 0., &na::vec3(1., 0., 0.))
      * na::rotation(std::f32::consts::PI, &na::vec3(0., 1., 0.))
      * na::rotation(std::f32::consts::PI, &na::vec3(0., 0., 1.));
    let translation = na::translation(&na::vec3(0., 0., -500.));
    let model = translation * rotation * scale * origin_shift;

    let model_view_projection = projection * view * model;

    gl.uniform_matrix4fv_with_f32_array(
      Some(&self.mvp_location),
      false,
      &model_view_projection.as_slice(),
    );

    gl.draw_arrays(GL::TRIANGLES, 0, self.num_vertices);
  }
}

fn f_shape_3d_positions() -> [f32; 288] {
  [
    0., 0., 0., 0., 150., 0., 30., 0., 0., 0., 150., 0., 30., 150., 0., 30., 0., 0., 30., 0., 0.,
    30., 30., 0., 100., 0., 0., 30., 30., 0., 100., 30., 0., 100., 0., 0., 30., 60., 0., 30., 90.,
    0., 67., 60., 0., 30., 90., 0., 67., 90., 0., 67., 60., 0., 0., 0., 30., 30., 0., 30., 0.,
    150., 30., 0., 150., 30., 30., 0., 30., 30., 150., 30., 30., 0., 30., 100., 0., 30., 30., 30.,
    30., 30., 30., 30., 100., 0., 30., 100., 30., 30., 30., 60., 30., 67., 60., 30., 30., 90., 30.,
    30., 90., 30., 67., 60., 30., 67., 90., 30., 0., 0., 0., 100., 0., 0., 100., 0., 30., 0., 0.,
    0., 100., 0., 30., 0., 0., 30., 100., 0., 0., 100., 30., 0., 100., 30., 30., 100., 0., 0.,
    100., 30., 30., 100., 0., 30., 30., 30., 0., 30., 30., 30., 100., 30., 30., 30., 30., 0., 100.,
    30., 30., 100., 30., 0., 30., 30., 0., 30., 60., 30., 30., 30., 30., 30., 30., 0., 30., 60.,
    0., 30., 60., 30., 30., 60., 0., 67., 60., 30., 30., 60., 30., 30., 60., 0., 67., 60., 0., 67.,
    60., 30., 67., 60., 0., 67., 90., 30., 67., 60., 30., 67., 60., 0., 67., 90., 0., 67., 90.,
    30., 30., 90., 0., 30., 90., 30., 67., 90., 30., 30., 90., 0., 67., 90., 30., 67., 90., 0.,
    30., 90., 0., 30., 150., 30., 30., 90., 30., 30., 90., 0., 30., 150., 0., 30., 150., 30., 0.,
    150., 0., 0., 150., 30., 30., 150., 30., 0., 150., 0., 30., 150., 30., 30., 150., 0., 0., 0.,
    0., 0., 0., 30., 0., 150., 30., 0., 0., 0., 0., 150., 30., 0., 150., 0.,
  ]
}

fn f_shape_3d_colors() -> [u8; 288] {
  [
    200, 70, 120, 200, 70, 120, 200, 70, 120, 200, 70, 120, 200, 70, 120, 200, 70, 120, 200, 70,
    120, 200, 70, 120, 200, 70, 120, 200, 70, 120, 200, 70, 120, 200, 70, 120, 200, 70, 120, 200,
    70, 120, 200, 70, 120, 200, 70, 120, 200, 70, 120, 200, 70, 120, 80, 70, 200, 80, 70, 200, 80,
    70, 200, 80, 70, 200, 80, 70, 200, 80, 70, 200, 80, 70, 200, 80, 70, 200, 80, 70, 200, 80, 70,
    200, 80, 70, 200, 80, 70, 200, 80, 70, 200, 80, 70, 200, 80, 70, 200, 80, 70, 200, 80, 70, 200,
    80, 70, 200, 70, 200, 210, 70, 200, 210, 70, 200, 210, 70, 200, 210, 70, 200, 210, 70, 200,
    210, 200, 200, 70, 200, 200, 70, 200, 200, 70, 200, 200, 70, 200, 200, 70, 200, 200, 70, 210,
    100, 70, 210, 100, 70, 210, 100, 70, 210, 100, 70, 210, 100, 70, 210, 100, 70, 210, 160, 70,
    210, 160, 70, 210, 160, 70, 210, 160, 70, 210, 160, 70, 210, 160, 70, 70, 180, 210, 70, 180,
    210, 70, 180, 210, 70, 180, 210, 70, 180, 210, 70, 180, 210, 100, 70, 210, 100, 70, 210, 100,
    70, 210, 100, 70, 210, 100, 70, 210, 100, 70, 210, 76, 210, 100, 76, 210, 100, 76, 210, 100,
    76, 210, 100, 76, 210, 100, 76, 210, 100, 140, 210, 80, 140, 210, 80, 140, 210, 80, 140, 210,
    80, 140, 210, 80, 140, 210, 80, 90, 130, 110, 90, 130, 110, 90, 130, 110, 90, 130, 110, 90,
    130, 110, 90, 130, 110, 160, 160, 220, 160, 160, 220, 160, 160, 220, 160, 160, 220, 160, 160,
    220, 160, 160, 220,
  ]
}
