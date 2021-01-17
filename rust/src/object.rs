use super::model::*;
use nalgebra_glm as na;
use std::collections::HashMap;
use std::rc::Rc;
use web_sys::WebGlRenderingContext as GL;
use web_sys::{WebGlTexture, WebGlUniformLocation};

pub struct Object {
  pub model: Rc<Model>,
  pub color_map: WebGlTexture,    // Move to model?
  pub specular_map: WebGlTexture, // ^
  pub normal_map: WebGlTexture,   // ^
  pub scale: na::Mat4,
  pub rotation: na::Mat4,
  pub translation: na::Mat4,
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
