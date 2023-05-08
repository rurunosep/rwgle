use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::WebGlRenderingContext as GL;
use web_sys::{Response, WebGlTexture};

pub async fn fetch_resource_as_array_buffer(url: &str) -> js_sys::ArrayBuffer {
    let response = JsFuture::from(web_sys::window().unwrap().fetch_with_str(url))
        .await
        .unwrap()
        .dyn_into::<Response>()
        .unwrap();
    let buffer = JsFuture::from(response.array_buffer().unwrap())
        .await
        .unwrap()
        .dyn_into::<js_sys::ArrayBuffer>()
        .unwrap();

    buffer
}

// Return a new texture filled with placeholder data and then call a
// JS func that will fetch the source image and fill the texture
// with new data once it's ready
//
// TODO: Do the async work here in Rust, without a call to JS
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
