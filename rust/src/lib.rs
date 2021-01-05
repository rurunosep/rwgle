use nalgebra_glm as na;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::WebGlRenderingContext as GL;
use web_sys::{HtmlCanvasElement, WebGlBuffer, WebGlProgram, WebGlShader, WebGlUniformLocation};

// web_sys::console::log_1(&"hello".into());

#[allow(dead_code)]
#[wasm_bindgen]
pub struct RustWebGLEngine {
    canvas: HtmlCanvasElement,
    gl: GL,
    program: WebGlProgram,
    num_vertices: i32,
    positions_buffer: WebGlBuffer,
    colors_buffer: WebGlBuffer,
    transform_location: WebGlUniformLocation,
}

#[wasm_bindgen]
impl RustWebGLEngine {
    #[wasm_bindgen(constructor)]
    pub fn new(canvas: JsValue) -> Result<RustWebGLEngine, JsValue> {
        #[cfg(debug_assertions)]
        console_error_panic_hook::set_once();

        let canvas = canvas.dyn_into::<HtmlCanvasElement>()?;
        let gl = canvas.get_context("webgl")?.unwrap().dyn_into::<GL>()?;

        gl.clear_color(0.0, 0.0, 0.0, 1.0);

        let program = link_program(
            &gl,
            include_str!("./shaders/2d_gradient.vert"),
            include_str!("./shaders/2d_gradient.frag"),
        )?;

        let transform_location = gl.get_uniform_location(&program, "transform").unwrap();

        let num_vertices = 3;

        #[rustfmt::skip]
        let positions: [f32; 6] = [
            0.0, 0.7,
            -0.61, -0.35,
            0.61, -0.35
        ];
        let positions_buffer = gl.create_buffer().unwrap();
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&positions_buffer));
        unsafe {
            let positions_array = js_sys::Float32Array::view(&positions);
            gl.buffer_data_with_array_buffer_view(
                GL::ARRAY_BUFFER,
                &positions_array,
                GL::STATIC_DRAW,
            );
        }

        #[rustfmt::skip]
        let colors: [f32; 9] = [
            1.0, 0.0, 0.0,
            0.0, 1.0, 0.0,
            0.0, 0.0, 1.0
        ];
        let colors_buffer = gl.create_buffer().unwrap();
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&colors_buffer));
        unsafe {
            let positions_array = js_sys::Float32Array::view(&colors);
            gl.buffer_data_with_array_buffer_view(
                GL::ARRAY_BUFFER,
                &positions_array,
                GL::STATIC_DRAW,
            );
        }

        Ok(RustWebGLEngine {
            canvas,
            gl,
            program,
            num_vertices,
            positions_buffer,
            colors_buffer,
            transform_location,
        })
    }

    pub fn render(&self) {
        let gl = &self.gl;

        gl.use_program(Some(&self.program));

        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&self.positions_buffer));
        gl.vertex_attrib_pointer_with_i32(0, 2, GL::FLOAT, false, 0, 0);
        gl.enable_vertex_attrib_array(0);

        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&self.colors_buffer));
        gl.vertex_attrib_pointer_with_i32(1, 3, GL::FLOAT, false, 0, 0);
        gl.enable_vertex_attrib_array(1);

        let time = web_sys::window().unwrap().performance().unwrap().now();
        let angle = (time % 10000. / 10000.) as f32 * (std::f32::consts::PI * 2.);
        let rotation = na::rotation(angle, &na::vec3(0.0, 0.0, -1.0));
        gl.uniform_matrix4fv_with_f32_array(
            Some(&self.transform_location),
            false,
            rotation.as_slice(),
        );

        gl.clear(GL::COLOR_BUFFER_BIT);
        gl.draw_arrays(GL::TRIANGLES, 0, self.num_vertices);
    }
}

fn link_program(
    gl: &GL,
    vertex_source: &str,
    fragment_source: &str,
) -> Result<WebGlProgram, String> {
    let program = gl
        .create_program()
        .ok_or_else(|| String::from("Error creating shader program"))?;

    let vertex_shader = compile_shader(&gl, GL::VERTEX_SHADER, vertex_source)?;
    let fragment_shader = compile_shader(&gl, GL::FRAGMENT_SHADER, fragment_source)?;

    gl.attach_shader(&program, &vertex_shader);
    gl.attach_shader(&program, &fragment_shader);
    gl.link_program(&program);

    if gl
        .get_program_parameter(&program, GL::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(program)
    } else {
        Err(gl
            .get_program_info_log(&program)
            .unwrap_or_else(|| String::from("Unknown error linking shader program")))
    }
}

fn compile_shader(gl: &GL, shader_type: u32, source: &str) -> Result<WebGlShader, String> {
    let shader = gl
        .create_shader(shader_type)
        .ok_or_else(|| String::from("Error creating shader"))?;
    gl.shader_source(&shader, &source);
    gl.compile_shader(&shader);

    if gl
        .get_shader_parameter(&shader, GL::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(shader)
    } else {
        Err(gl
            .get_shader_info_log(&shader)
            .unwrap_or_else(|| String::from("Unknown error compiling shader")))
    }
}
