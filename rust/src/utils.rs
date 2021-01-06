use web_sys::WebGlRenderingContext as GL;
use web_sys::{WebGlProgram, WebGlShader};

// TODO: only check compile status if linking fails?

pub fn link_program(
  gl: &GL,
  vertex_source: &str,
  fragment_source: &str,
) -> Result<WebGlProgram, String> {
  let program = gl.create_program().unwrap();

  let vertex_shader = compile_shader(&gl, GL::VERTEX_SHADER, vertex_source)?;
  let fragment_shader = compile_shader(&gl, GL::FRAGMENT_SHADER, fragment_source)?;

  gl.attach_shader(&program, &vertex_shader);
  gl.attach_shader(&program, &fragment_shader);
  gl.link_program(&program);

  if gl
    .get_program_parameter(&program, GL::LINK_STATUS)
    .as_bool()
    .unwrap()
  {
    Ok(program)
  } else {
    Err(gl.get_program_info_log(&program).unwrap())
  }
}

pub fn compile_shader(gl: &GL, shader_type: u32, source: &str) -> Result<WebGlShader, String> {
  let shader = gl.create_shader(shader_type).unwrap();
  gl.shader_source(&shader, &source);
  gl.compile_shader(&shader);

  if gl
    .get_shader_parameter(&shader, GL::COMPILE_STATUS)
    .as_bool()
    .unwrap()
  {
    Ok(shader)
  } else {
    Err(gl.get_shader_info_log(&shader).unwrap())
  }
}
