use std::cell::RefCell;
use std::rc::Rc;

use failure::{err_msg, Error};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{
    HtmlCanvasElement, HtmlElement, HtmlInputElement, MouseEvent, WebGlRenderingContext,
};

use blocks::BlockLighting;
use util::{handle_error, js_err, show_element};

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
    block_lighting: BlockLighting,
    width: u32,
    height: u32,
    mode: Mode,
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

        show_element(&elements.mode_section)?;

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

        let block_lighting = BlockLighting::new(context.clone(), 24, 18)?;

        let demo = Rc::new(RefCell::new(Demo {
            elements,
            context,
            block_lighting,
            width,
            height,
            mode: Mode::SolidBlock,
        }));

        Demo::attach_callbacks(demo)?;

        Ok(())
    }

    fn attach_callbacks(demo: Rc<RefCell<Demo>>) -> Result<(), Error> {
        let elements = demo.borrow().elements.clone();

        let callback_demo = demo.clone();
        let mouse_down_callback = Closure::wrap(Box::new(move |mouse_event| {
            handle_error("mouse down callback", || {
                callback_demo.borrow_mut().mouse_down(mouse_event)
            });
        }) as Box<FnMut(MouseEvent)>);
        AsRef::<HtmlElement>::as_ref(&elements.canvas)
            .set_onmousedown(Some(mouse_down_callback.as_ref().unchecked_ref()));
        mouse_down_callback.forget();

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

    fn mouse_down(&mut self, mouse_event: MouseEvent) -> Result<(), Error> {
        self.handle_mouse_event(mouse_event.x(), mouse_event.y())
    }

    fn mouse_move(&mut self, mouse_event: MouseEvent) -> Result<(), Error> {
        if (mouse_event.buttons() & 1) != 0 {
            self.handle_mouse_event(mouse_event.x(), mouse_event.y())?;
        }
        Ok(())
    }

    fn set_mode(&mut self, mode: Mode) -> Result<(), Error> {
        self.mode = mode;
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
        self.context.clear_color(0.0, 0.0, 0.0, 1.0);
        self.context.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);

        self.block_lighting.draw()?;

        Ok(())
    }

    fn handle_mouse_event(&mut self, x: i32, y: i32) -> Result<(), Error> {
        let x = x as f32 / self.width as f32;
        let y = 1.0 - y as f32 / self.height as f32;

        if x < 0.0 || x > 1.0 || y < 0.0 || y > 1.0 {
            return Ok(());
        }

        let (xcount, ycount) = self.block_lighting.block_count();
        let xi = (x * xcount as f32).floor() as u32;
        let yi = (y * ycount as f32).floor() as u32;

        let changed = match self.mode {
            Mode::SolidBlock => self.block_lighting.set_block_state(xi, yi, true),
            Mode::Erase => self.block_lighting.set_block_state(xi, yi, false),
            _ => false,
        };

        if changed {
            self.draw()?;
        }

        Ok(())
    }
}
