use super::mesh::*;
use std::rc::Rc;
use web_sys::WebGlRenderingContext as GL;
use web_sys::WebGlTexture;

pub struct Model {
  pub mesh: Rc<Mesh>,
  pub color_map: WebGlTexture,
  pub specular_map: WebGlTexture,
  pub normal_map: WebGlTexture,
}

impl Model {
  pub fn render(&self, gl: &GL) {
    gl.active_texture(GL::TEXTURE0);
    gl.bind_texture(GL::TEXTURE_2D, Some(&self.color_map));
    gl.active_texture(GL::TEXTURE1);
    gl.bind_texture(GL::TEXTURE_2D, Some(&self.specular_map));
    gl.active_texture(GL::TEXTURE2);
    gl.bind_texture(GL::TEXTURE_2D, Some(&self.normal_map));

    self.mesh.render(&gl);
  }
}
