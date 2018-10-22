use std::cell::RefCell;
use std::rc::Rc;

use failure::{err_msg, Error};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{
    HtmlCanvasElement, HtmlElement, HtmlInputElement, MouseEvent, WebGlRenderingContext,
};

use glutil::{compile_shader, link_program};
use util::{handle_error, js_err};

#[derive(Clone)]
pub struct DemoElements {
    pub canvas: HtmlCanvasElement,

    pub mode_section: HtmlElement,
    pub solid_block_mode_radio: HtmlElement,
    pub light_block_mode_radio: HtmlElement,
    pub point_light_mode_radio: HtmlElement,
    pub erase_mode_radio: HtmlElement,

    pub angle_section: HtmlElement,
    pub angle_slider: HtmlInputElement,

    pub pointiness_section: HtmlElement,
    pub pointiness_slider: HtmlInputElement,

    pub spread_section: HtmlElement,
    pub spread_steps_slider: HtmlInputElement,

    pub clear_section: HtmlElement,
    pub clear_button: HtmlElement,

    pub algorithm_section: HtmlElement,
    pub advance_button: HtmlElement,
    pub reset_button: HtmlElement,
}

pub struct Demo {
    elements: DemoElements,
    context: WebGlRenderingContext,
}

#[derive(Debug)]
enum Mode {
    SolidBlock,
    LightBlock,
    PointLight,
    Erase,
}

impl Demo {
    pub fn init(elements: DemoElements) -> Result<(), Error> {
        let width = AsRef::<HtmlElement>::as_ref(&elements.canvas).offset_width() as u32;
        let height = AsRef::<HtmlElement>::as_ref(&elements.canvas).offset_height() as u32;

        elements.canvas.set_width(width);
        elements.canvas.set_height(height);

        let context = elements
            .canvas
            .get_context("webgl")
            .map_err(js_err)?
            .ok_or_else(|| err_msg("no webgl context available"))?
            .dyn_into::<WebGlRenderingContext>()
            .map_err(|_| err_msg("webgl context is incorrect type"))?;
        context.viewport(0, 0, width as i32, height as i32);

        let vert_shader = compile_shader(
            &context,
            WebGlRenderingContext::VERTEX_SHADER,
            VERTEX_SHADER,
        )?;
        let frag_shader = compile_shader(
            &context,
            WebGlRenderingContext::FRAGMENT_SHADER,
            FRAGMENT_SHADER,
        )?;
        let program = link_program(&context, [vert_shader, frag_shader].iter())?;
        context.use_program(Some(&program));

        let demo = Rc::new(RefCell::new(Demo { elements, context }));

        Demo::attach_callbacks(demo)?;

        Ok(())
    }

    fn attach_callbacks(demo: Rc<RefCell<Demo>>) -> Result<(), Error> {
        let elements = demo.borrow().elements.clone();

        let callback_demo = demo.clone();
        let mouse_move_callback = Closure::wrap(Box::new(move |mouse_event| {
            handle_error("mouse move callback", || {
                callback_demo.borrow_mut().mouse_move(mouse_event)
            });
        }) as Box<FnMut(MouseEvent)>);
        AsRef::<HtmlElement>::as_ref(&elements.canvas)
            .set_onmousemove(Some(mouse_move_callback.as_ref().unchecked_ref()));
        mouse_move_callback.forget();

        let callback_demo = demo.clone();
        let solid_block_callback = Closure::wrap(Box::new(move || {
            handle_error("mode change callback", || {
                callback_demo.borrow_mut().set_mode(Mode::SolidBlock)
            });
        }) as Box<FnMut()>);
        elements
            .solid_block_mode_radio
            .set_onchange(Some(solid_block_callback.as_ref().unchecked_ref()));
        solid_block_callback.forget();

        let callback_demo = demo.clone();
        let light_block_callback = Closure::wrap(Box::new(move || {
            handle_error("mode change callback", || {
                callback_demo.borrow_mut().set_mode(Mode::LightBlock)
            });
        }) as Box<FnMut()>);
        elements
            .light_block_mode_radio
            .set_onchange(Some(light_block_callback.as_ref().unchecked_ref()));
        light_block_callback.forget();

        let callback_demo = demo.clone();
        let point_light_callback = Closure::wrap(Box::new(move || {
            handle_error("mode change callback", || {
                callback_demo.borrow_mut().set_mode(Mode::PointLight)
            });
        }) as Box<FnMut()>);
        elements
            .point_light_mode_radio
            .set_onchange(Some(point_light_callback.as_ref().unchecked_ref()));
        point_light_callback.forget();

        let callback_demo = demo.clone();
        let erase_mode_callback = Closure::wrap(Box::new(move || {
            handle_error("mode change callback", || {
                callback_demo.borrow_mut().set_mode(Mode::Erase)
            });
        }) as Box<FnMut()>);
        elements
            .erase_mode_radio
            .set_onchange(Some(erase_mode_callback.as_ref().unchecked_ref()));
        erase_mode_callback.forget();

        let callback_demo = demo.clone();
        let angle_callback = Closure::wrap(Box::new(move || {
            handle_error("set angle callback", || {
                let mut callback_demo = callback_demo.borrow_mut();
                let angle = callback_demo.elements.angle_slider.value().parse()?;
                callback_demo.set_angle(angle)?;
                Ok(())
            });
        }) as Box<FnMut()>);
        AsRef::<HtmlElement>::as_ref(&elements.angle_slider)
            .set_onchange(Some(angle_callback.as_ref().unchecked_ref()));
        angle_callback.forget();

        let callback_demo = demo.clone();
        let pointiness_callback = Closure::wrap(Box::new(move || {
            handle_error("set pointiness callback", || {
                let mut callback_demo = callback_demo.borrow_mut();
                let pointiness = callback_demo.elements.pointiness_slider.value().parse()?;
                callback_demo.set_pointiness(pointiness)?;
                Ok(())
            });
        }) as Box<FnMut()>);
        AsRef::<HtmlElement>::as_ref(&elements.pointiness_slider)
            .set_onchange(Some(pointiness_callback.as_ref().unchecked_ref()));
        pointiness_callback.forget();

        let callback_demo = demo.clone();
        let spread_steps_callback = Closure::wrap(Box::new(move || {
            handle_error("set spread steps callback", || {
                let mut callback_demo = callback_demo.borrow_mut();
                let spread_steps = callback_demo.elements.spread_steps_slider.value().parse()?;
                callback_demo.set_spread_steps(spread_steps)?;
                Ok(())
            });
        }) as Box<FnMut()>);
        AsRef::<HtmlElement>::as_ref(&elements.spread_steps_slider)
            .set_onchange(Some(spread_steps_callback.as_ref().unchecked_ref()));
        spread_steps_callback.forget();

        let callback_demo = demo.clone();
        let clear_callback = Closure::wrap(Box::new(move || {
            handle_error("clear callback", || callback_demo.borrow_mut().clear());
        }) as Box<FnMut()>);
        elements
            .clear_button
            .set_onclick(Some(clear_callback.as_ref().unchecked_ref()));
        clear_callback.forget();

        let callback_demo = demo.clone();
        let advance_callback = Closure::wrap(Box::new(move || {
            handle_error("advance callback", || callback_demo.borrow_mut().advance());
        }) as Box<FnMut()>);
        elements
            .advance_button
            .set_onclick(Some(advance_callback.as_ref().unchecked_ref()));
        advance_callback.forget();

        let callback_demo = demo.clone();
        let reset_callback = Closure::wrap(Box::new(move || {
            handle_error("reset callback", || callback_demo.borrow_mut().reset());
        }) as Box<FnMut()>);
        elements
            .reset_button
            .set_onclick(Some(reset_callback.as_ref().unchecked_ref()));
        reset_callback.forget();

        demo.borrow_mut().draw()?;

        Ok(())
    }

    fn mouse_move(&mut self, _mouse_event: MouseEvent) -> Result<(), Error> {
        Ok(())
    }

    fn set_mode(&mut self, _mode: Mode) -> Result<(), Error> {
        Ok(())
    }

    fn set_angle(&mut self, _amt: i32) -> Result<(), Error> {
        Ok(())
    }

    fn set_pointiness(&mut self, _amt: i32) -> Result<(), Error> {
        Ok(())
    }

    fn set_spread_steps(&mut self, _amt: i32) -> Result<(), Error> {
        Ok(())
    }

    fn clear(&mut self) -> Result<(), Error> {
        Ok(())
    }

    fn advance(&mut self) -> Result<(), Error> {
        Ok(())
    }

    fn reset(&mut self) -> Result<(), Error> {
        Ok(())
    }

    fn draw(&mut self) -> Result<(), Error> {
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

        let buffer = self
            .context
            .create_buffer()
            .ok_or_else(|| err_msg("could not create buffer"))?;
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

        Ok(())
    }
}

const VERTEX_SHADER: &str = r#"
    precision mediump float;

    attribute vec2 a_position;
    attribute vec3 a_color;

    varying vec3 v_color;

    void main() {
        v_color = a_color;
        gl_Position = vec4(a_position, 0.0, 1.0);
    }
"#;

const FRAGMENT_SHADER: &str = r#"
    precision mediump float;

    varying vec3 v_color;

    void main() {
        gl_FragColor = vec4(v_color, 1.0);
    }
"#;
