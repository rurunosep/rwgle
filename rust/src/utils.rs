use std::collections::HashMap;
use web_sys::WebGlRenderingContext as GL;
use web_sys::{WebGlBuffer, WebGlProgram, WebGlShader, WebGlUniformLocation};

pub struct Attribute {
  pub index: u32,
  pub size: i32,
  pub type_: u32,
  pub buffer: WebGlBuffer,
}

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

pub fn get_attributes(gl: &GL, program: &WebGlProgram) -> HashMap<String, Attribute> {
  let num_attributes = gl
    .get_program_parameter(&program, GL::ACTIVE_ATTRIBUTES)
    .as_f64()
    .unwrap() as u32;
  let mut map = HashMap::new();
  for index in 0..num_attributes {
    let info = gl.get_active_attrib(&program, index).unwrap();
    let (size, type_) = match info.type_() {
      GL::FLOAT_VEC3 => (3, GL::FLOAT),
      GL::FLOAT_VEC2 => (2, GL::FLOAT),
      x => panic!("No match for attribute type: {}", x),
    };
    map.insert(
      info.name(),
      Attribute {
        index,
        size,
        type_,
        buffer: gl.create_buffer().unwrap(),
      },
    );
  }
  map
}

pub fn get_uniform_locations(
  gl: &GL,
  program: &WebGlProgram,
) -> HashMap<String, WebGlUniformLocation> {
  let num_uniforms = gl
    .get_program_parameter(&program, GL::ACTIVE_UNIFORMS)
    .as_f64()
    .unwrap() as u32;
  let mut map = HashMap::new();
  for i in 0..num_uniforms {
    let name = gl.get_active_uniform(&program, i).unwrap().name();
    let location = gl.get_uniform_location(&program, &name).unwrap();
    map.insert(name, location);
  }
  map
}

pub fn buffer_attribute_data(
  gl: &GL,
  attributes: &HashMap<String, Attribute>,
  name: &str,
  data: &[f32],
) {
  let buffer = &attributes.get(name).unwrap().buffer;
  gl.bind_buffer(GL::ARRAY_BUFFER, Some(&buffer));
  unsafe {
    let array = js_sys::Float32Array::view(&data);
    gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &array, GL::STATIC_DRAW);
  }
}

pub fn buffer_index_data(gl: &GL, data: &[u16]) -> (WebGlBuffer, i32) {
  let buffer = gl.create_buffer().unwrap();
  gl.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, Some(&buffer));
  unsafe {
    let array = js_sys::Uint16Array::view(&data);
    gl.buffer_data_with_array_buffer_view(GL::ELEMENT_ARRAY_BUFFER, &array, GL::STATIC_DRAW);
  }
  (buffer, data.len() as i32)
}

pub fn set_attrib_pointers(gl: &GL, attributes: &HashMap<String, Attribute>) {
  for a in attributes.values() {
    gl.bind_buffer(GL::ARRAY_BUFFER, Some(&a.buffer));
    gl.vertex_attrib_pointer_with_i32(a.index, a.size, a.type_, false, 0, 0);
    gl.enable_vertex_attrib_array(a.index);
  }
}
