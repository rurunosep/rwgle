use nalgebra_glm as na;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use web_sys::WebGlRenderingContext as GL;
use web_sys::{WebGlBuffer, WebGlProgram, WebGlShader, WebGlTexture, WebGlUniformLocation};

pub struct Attribute {
  pub name: String,
  pub index: u32,
  pub size: i32,
  pub type_: u32,
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
        name: info.name(),
        index,
        size,
        type_,
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

pub fn buffer_index_data(gl: &GL, data: &[u16]) -> (WebGlBuffer, i32) {
  let buffer = gl.create_buffer().unwrap();
  gl.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, Some(&buffer));
  unsafe {
    let array = js_sys::Uint16Array::view(&data);
    gl.buffer_data_with_array_buffer_view(GL::ELEMENT_ARRAY_BUFFER, &array, GL::STATIC_DRAW);
  }
  (buffer, data.len() as i32)
}

// TODO: nicer
pub fn calc_tangents_bitangents(
  indices: &[u16],
  positions: &[f32],
  texcoords: &[f32],
) -> (Vec<f32>, Vec<f32>) {
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

  (tangents, bitangents)
}

// Return a new texture filled with placeholder data and then call a
// JS func that will fetch the source image and fill the texture
// with new data once it's ready
pub fn load_texture(gl: &GL, source_url: &str) -> WebGlTexture {
  let texture = gl.create_texture().unwrap();

  // Fill texture with placeholder data
  gl.active_texture(GL::TEXTURE0);
  gl.bind_texture(GL::TEXTURE_2D, Some(&texture));
  gl.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(
    GL::TEXTURE_2D,
    0,
    GL::RGBA as i32,
    1,
    1,
    0,
    GL::RGBA,
    GL::UNSIGNED_BYTE,
    Some(&[0, 0, 255, 255]),
  )
  .unwrap();
  gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_MAG_FILTER, GL::LINEAR as i32);
  gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_MIN_FILTER, GL::LINEAR as i32);

  // Asynchronously fill texture with image data with call to JS
  load_texture_image(&gl, &texture, &source_url);

  texture
}

#[wasm_bindgen(raw_module = "../js/index.js")]
extern "C" {
  #[wasm_bindgen(js_name = loadTextureImage)]
  fn load_texture_image(gl: &GL, texture: &WebGlTexture, source_url: &str);
}
