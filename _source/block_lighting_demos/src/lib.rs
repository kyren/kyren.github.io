extern crate js_sys;
extern crate wasm_bindgen;
extern crate web_sys;

use std::cell::RefCell;
use std::rc::Rc;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{console, HtmlElement, WebGlProgram, WebGlRenderingContext, WebGlShader, Window};

#[wasm_bindgen]
pub fn init() {
    fn draw(window: Window, state: Rc<RefCell<State>>, time: f64) {
        state.borrow_mut().draw(time);

        let callback_window = window.clone();
        let callback = Closure::wrap(Box::new(move |time| {
            draw(callback_window.clone(), state.clone(), time);
        }) as Box<FnMut(f64)>);
        window
            .request_animation_frame(callback.as_ref().unchecked_ref())
            .unwrap();
        callback.forget();
    }

    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();

    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    let state = Rc::new(RefCell::new(State::init(canvas)));

    let clear_state = state.clone();
    let clear_callback = Closure::wrap(Box::new(move || {
        clear_state.borrow_mut().clear();
    }) as Box<FnMut()>);

    document
        .get_element_by_id("clear")
        .unwrap()
        .dyn_ref::<HtmlElement>()
        .unwrap()
        .set_onclick(Some(clear_callback.as_ref().unchecked_ref()));
    clear_callback.forget();

    draw(window, state, 0.0);
}

struct State {
    context: WebGlRenderingContext,
}

impl State {
    fn init(canvas: web_sys::HtmlCanvasElement) -> State {
        let width = canvas
            .dyn_ref::<web_sys::HtmlElement>()
            .unwrap()
            .offset_width() as u32;
        let height = canvas
            .dyn_ref::<web_sys::HtmlElement>()
            .unwrap()
            .offset_height() as u32;

        canvas.set_width(width);
        canvas.set_height(height);

        let context = canvas
            .get_context("webgl")
            .unwrap()
            .unwrap()
            .dyn_into::<WebGlRenderingContext>()
            .unwrap();
        context.viewport(0, 0, width as i32, height as i32);

        let vert_shader = compile_shader(
            &context,
            WebGlRenderingContext::VERTEX_SHADER,
            r#"
                precision mediump float;

                attribute vec2 a_position;
                attribute vec3 a_color;

                varying vec3 v_color;

                void main() {
                    v_color = a_color;
                    gl_Position = vec4(a_position, 0.0, 1.0);
                }
            "#,
        ).unwrap();
        let frag_shader = compile_shader(
            &context,
            WebGlRenderingContext::FRAGMENT_SHADER,
            r#"
                precision mediump float;

                varying vec3 v_color;

                void main() {
                    gl_FragColor = vec4(v_color, 1.0);
                }
            "#,
        ).unwrap();
        let program = link_program(&context, [vert_shader, frag_shader].iter()).unwrap();
        context.use_program(Some(&program));

        State { context }
    }

    fn draw(&mut self, _time: f64) {
        self.context.clear(
            WebGlRenderingContext::COLOR_BUFFER_BIT | WebGlRenderingContext::DEPTH_BUFFER_BIT,
        );

        let vertices = [
            -0.7, -0.7, 1.0, 0.0, 0.0, 0.7, -0.7, 0.0, 1.0, 0.0, 0.0, 0.7, 0.0, 0.0, 1.0,
        ];
        let vert_array =
            js_sys::Float32Array::new(&wasm_bindgen::JsValue::from(vertices.len() as u32));
        for (i, f) in vertices.iter().enumerate() {
            vert_array.fill(*f, i as u32, (i + 1) as u32);
        }

        let buffer = self.context.create_buffer().unwrap();
        self.context
            .bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&buffer));
        self.context.buffer_data_with_opt_array_buffer(
            WebGlRenderingContext::ARRAY_BUFFER,
            Some(&vert_array.buffer()),
            WebGlRenderingContext::STATIC_DRAW,
        );
        self.context.vertex_attrib_pointer_with_i32(
            0,
            2,
            WebGlRenderingContext::FLOAT,
            false,
            5 * 4,
            0 * 4,
        );
        self.context.enable_vertex_attrib_array(0);

        self.context.vertex_attrib_pointer_with_i32(
            1,
            3,
            WebGlRenderingContext::FLOAT,
            false,
            5 * 4,
            2 * 4,
        );
        self.context.enable_vertex_attrib_array(1);

        self.context.clear_color(0.0, 0.0, 0.0, 1.0);
        self.context.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);

        self.context.draw_arrays(
            WebGlRenderingContext::TRIANGLES,
            0,
            (vertices.len() / 5) as i32,
        );
    }

    fn clear(&mut self) {
        console::log_1(&"clear button clicked".into());
    }
}

fn compile_shader(
    context: &WebGlRenderingContext,
    shader_type: u32,
    source: &str,
) -> Result<WebGlShader, String> {
    let shader = context
        .create_shader(shader_type)
        .ok_or_else(|| String::from("Unable to create shader object"))?;
    context.shader_source(&shader, source);
    context.compile_shader(&shader);

    if context
        .get_shader_parameter(&shader, WebGlRenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(shader)
    } else {
        Err(context
            .get_shader_info_log(&shader)
            .unwrap_or_else(|| "Unknown error creating shader".into()))
    }
}

fn link_program<'a, T: IntoIterator<Item = &'a WebGlShader>>(
    context: &WebGlRenderingContext,
    shaders: T,
) -> Result<WebGlProgram, String> {
    let program = context
        .create_program()
        .ok_or_else(|| String::from("Unable to create shader object"))?;
    for shader in shaders {
        context.attach_shader(&program, shader)
    }
    context.link_program(&program);

    if context
        .get_program_parameter(&program, WebGlRenderingContext::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(program)
    } else {
        Err(context
            .get_program_info_log(&program)
            .unwrap_or_else(|| "Unknown error creating program object".into()))
    }
}
