use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlCanvasElement;
use web_sys::WebGlRenderingContext as GL;

mod simple_3d;
mod utils;
use simple_3d::*;

// web_sys::console::log_1(&format!("{}").into());

// Smaller but less efficient memory allocator.
// Remove if performance becomes a bigger concern.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct RustWebGLEngine {
    gl: GL,
    simple_3d: Simple3D,
}

#[wasm_bindgen]
impl RustWebGLEngine {
    #[wasm_bindgen(constructor)]
    pub fn new(canvas: JsValue) -> Result<RustWebGLEngine, JsValue> {
        #[cfg(debug_assertions)]
        console_error_panic_hook::set_once();

        let canvas = canvas.dyn_into::<HtmlCanvasElement>()?;
        let gl = canvas.get_context("webgl")?.unwrap().dyn_into::<GL>()?;

        let simple_3d = Simple3D::new(&gl)?;

        Ok(RustWebGLEngine { gl, simple_3d })
    }

    pub fn render(&mut self) {
        self.gl.enable(GL::CULL_FACE);
        self.gl.enable(GL::DEPTH_TEST);
        self.gl.clear_color(0.0, 0.0, 0.0, 1.0);
        self.gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);

        self.simple_3d.render(&self.gl);
    }
}
