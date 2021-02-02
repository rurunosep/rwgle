use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlCanvasElement;
use web_sys::WebGlRenderingContext as GL;

mod mesh;
mod model;
mod object;
mod renderer;
mod utils;
use renderer::*;

// web_sys::console::log_1(&format!("{}").into());

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct RustWebGLEngine {
    gl: GL,
    renderer: Renderer,
}

#[wasm_bindgen]
impl RustWebGLEngine {
    #[wasm_bindgen(constructor)]
    pub async fn new(canvas: JsValue) -> Result<RustWebGLEngine, JsValue> {
        #[cfg(debug_assertions)]
        console_error_panic_hook::set_once();

        let canvas = canvas.dyn_into::<HtmlCanvasElement>()?;
        let gl = canvas.get_context("webgl")?.unwrap().dyn_into::<GL>()?;

        let renderer = Renderer::new(&gl).await?;

        Ok(RustWebGLEngine { gl, renderer })
    }

    // If I make this async, it MUST take "self" NOT "&self"
    pub fn render(&mut self) {
        let canvas = self
            .gl
            .canvas()
            .unwrap()
            .dyn_into::<HtmlCanvasElement>()
            .unwrap();
        canvas.set_width(canvas.client_width() as u32);
        canvas.set_height(canvas.client_height() as u32);
        self.gl
            .viewport(0, 0, canvas.width() as i32, canvas.height() as i32);

        self.gl.enable(GL::CULL_FACE);
        self.gl.enable(GL::DEPTH_TEST);
        self.gl.clear_color(0.8, 0.8, 0.8, 1.);
        self.gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);

        self.renderer.render(&self.gl);
    }

    #[wasm_bindgen(js_name = rotateCameraLeft)]
    pub fn rotate_camera_left(&mut self) {
        self.renderer.rotate_camera_left();
    }

    #[wasm_bindgen(js_name = rotateCameraRight)]
    pub fn rotate_camera_right(&mut self) {
        self.renderer.rotate_camera_right();
    }
}
