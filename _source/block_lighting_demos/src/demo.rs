use std::cell::RefCell;
use std::f32;
use std::hash::Hasher;
use std::rc::Rc;

use failure::{err_msg, Error};
use twox_hash::XxHash;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{
    HtmlCanvasElement, HtmlElement, HtmlImageElement, HtmlInputElement, MouseEvent,
    WebGlRenderingContext,
};

use crate::renderer::Renderer;
use crate::util::{console_log, get_element, handle_error, js_err, show_element};

#[wasm_bindgen]
pub fn demo_init() {
    handle_error("initialization", || {
        let main_texture = get_element("main_texture")?;
        let canvas = get_element("canvas")?;

        let mode_section = get_element("mode_controls")?;
        let foreground_block_mode_radio = get_element("foreground_block_mode")?;
        let background_block_mode_radio = get_element("background_block_mode")?;
        let light_block_mode_radio = get_element("light_block_mode")?;
        let point_light_mode_radio = get_element("point_light_mode")?;
        let erase_mode_radio = get_element("erase_mode")?;

        let angle_section = get_element("angle_controls")?;
        let angle_slider = get_element("angle")?;

        let pointiness_section = get_element("pointiness_controls")?;
        let pointiness_slider = get_element("pointiness")?;

        let spread_section = get_element("spread_controls")?;
        let spread_steps_slider = get_element("spread_steps")?;

        let clear_section = get_element("clear_controls")?;
        let clear_button = get_element("clear")?;

        let algorithm_section = get_element("algorithm_controls")?;
        let advance_button = get_element("advance")?;
        let reset_button = get_element("reset")?;

        Demo::init(DemoElements {
            main_texture,
            canvas,

            mode_section,
            foreground_block_mode_radio,
            background_block_mode_radio,
            light_block_mode_radio,
            point_light_mode_radio,
            erase_mode_radio,

            angle_section,
            angle_slider,

            pointiness_section,
            pointiness_slider,

            spread_section,
            spread_steps_slider,

            clear_section,
            clear_button,

            algorithm_section,
            advance_button,
            reset_button,
        })?;

        Ok(())
    });
}

#[derive(Clone)]
struct DemoElements {
    main_texture: HtmlImageElement,
    canvas: HtmlCanvasElement,

    mode_section: HtmlElement,
    foreground_block_mode_radio: HtmlElement,
    background_block_mode_radio: HtmlElement,
    light_block_mode_radio: HtmlElement,
    point_light_mode_radio: HtmlElement,
    erase_mode_radio: HtmlElement,

    angle_section: HtmlElement,
    angle_slider: HtmlInputElement,

    pointiness_section: HtmlElement,
    pointiness_slider: HtmlInputElement,

    spread_section: HtmlElement,
    spread_steps_slider: HtmlInputElement,

    clear_section: HtmlElement,
    clear_button: HtmlElement,

    algorithm_section: HtmlElement,
    advance_button: HtmlElement,
    reset_button: HtmlElement,
}

struct Demo {
    elements: DemoElements,
    context: WebGlRenderingContext,
    width: u32,
    height: u32,

    mode: Mode,
    cursor: Cursor,
    block_state: Vec<BlockState>,
    point_lights: Vec<(f32, f32)>,
    renderer: Renderer,
}

#[derive(PartialEq, Eq, Debug)]
enum Mode {
    ForegroundBlock,
    BackgroundBlock,
    LightBlock,
    PointLight,
    Erase,
}

#[derive(PartialEq, Debug)]
enum Cursor {
    None,
    Block(u32, u32),
    ExistingLight(usize),
    NewLight(f32, f32),
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum BlockState {
    Empty,
    Foreground,
    Background,
    Light,
}

const BLOCK_TEX_COORDS: [(f32, f32, f32, f32); 4] = [
    (0.0 / 4.0, 1.0 / 4.0, 1.0 / 4.0, 0.0 / 4.0),
    (1.0 / 4.0, 1.0 / 4.0, 2.0 / 4.0, 0.0 / 4.0),
    (0.0 / 4.0, 2.0 / 4.0, 1.0 / 4.0, 1.0 / 4.0),
    (1.0 / 4.0, 2.0 / 4.0, 2.0 / 4.0, 1.0 / 4.0),
];

const GRASS_TEX_COORDS: [(f32, f32, f32, f32); 4] = [
    (2.0 / 4.0, 1.0 / 4.0, 3.0 / 4.0, 0.0 / 4.0),
    (3.0 / 4.0, 1.0 / 4.0, 4.0 / 4.0, 0.0 / 4.0),
    (2.0 / 4.0, 2.0 / 4.0, 3.0 / 4.0, 1.0 / 4.0),
    (3.0 / 4.0, 2.0 / 4.0, 4.0 / 4.0, 1.0 / 4.0),
];

const LIGHT_BLOCK_TEX_COORDS: [(f32, f32, f32, f32); 4] = [
    (0.0 / 4.0, 3.0 / 4.0, 1.0 / 4.0, 2.0 / 4.0),
    (1.0 / 4.0, 3.0 / 4.0, 2.0 / 4.0, 2.0 / 4.0),
    (0.0 / 4.0, 4.0 / 4.0, 1.0 / 4.0, 3.0 / 4.0),
    (1.0 / 4.0, 4.0 / 4.0, 2.0 / 4.0, 3.0 / 4.0),
];

const POINT_LIGHT_TEX_COORDS: (f32, f32, f32, f32) = (2.0 / 4.0, 3.0 / 4.0, 3.0 / 4.0, 2.0 / 4.0);
const BLOCK_SELECT_TEX_COORDS: (f32, f32, f32, f32) = (3.0 / 4.0, 3.0 / 4.0, 4.0 / 4.0, 2.0 / 4.0);
const LIGHT_SELECT_TEX_COORDS: (f32, f32, f32, f32) = (2.0 / 4.0, 4.0 / 4.0, 3.0 / 4.0, 3.0 / 4.0);
const BACKGROUND_TEX_COORDS: (f32, f32, f32, f32) = (3.0 / 4.0, 4.0 / 4.0, 4.0 / 4.0, 3.0 / 4.0);

const BLOCK_COUNT: (u32, u32) = (24, 18);
const BLOCK_SIZE: (f32, f32) = (1.0 / BLOCK_COUNT.0 as f32, 1.0 / BLOCK_COUNT.1 as f32);
const LIGHT_SELECTION_RADIUS: f32 = 1.0 / 30.0;

impl Demo {
    fn init(elements: DemoElements) -> Result<(), Error> {
        let width = AsRef::<HtmlElement>::as_ref(&elements.canvas).offset_width() as u32;
        let height = AsRef::<HtmlElement>::as_ref(&elements.canvas).offset_height() as u32;

        show_element(&elements.mode_section)?;
        // show_element(&elements.angle_section)?;
        // show_element(&elements.pointiness_section)?;
        // show_element(&elements.spread_section)?;
        // show_element(&elements.clear_section)?;
        // show_element(&elements.algorithm_section)?;

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
        context.disable(WebGlRenderingContext::DEPTH_TEST);
        context.enable(WebGlRenderingContext::BLEND);
        context.blend_func(
            WebGlRenderingContext::SRC_ALPHA,
            WebGlRenderingContext::ONE_MINUS_SRC_ALPHA,
        );
        context.blend_equation(WebGlRenderingContext::FUNC_ADD);

        let renderer = Renderer::new(context.clone(), elements.main_texture.clone())?;

        let demo = Rc::new(RefCell::new(Demo {
            elements,
            context,
            width,
            height,
            mode: Mode::ForegroundBlock,
            cursor: Cursor::None,
            block_state: vec![BlockState::Empty; BLOCK_COUNT.0 as usize * BLOCK_COUNT.1 as usize],
            point_lights: Vec::new(),
            renderer,
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
        let foreground_block_callback = Closure::wrap(Box::new(move || {
            handle_error("mode change callback", || {
                callback_demo.borrow_mut().set_mode(Mode::ForegroundBlock)
            });
        }) as Box<FnMut()>);
        elements
            .foreground_block_mode_radio
            .set_onchange(Some(foreground_block_callback.as_ref().unchecked_ref()));
        foreground_block_callback.forget();

        let callback_demo = demo.clone();
        let background_block_callback = Closure::wrap(Box::new(move || {
            handle_error("mode change callback", || {
                callback_demo.borrow_mut().set_mode(Mode::BackgroundBlock)
            });
        }) as Box<FnMut()>);
        elements
            .background_block_mode_radio
            .set_onchange(Some(background_block_callback.as_ref().unchecked_ref()));
        background_block_callback.forget();

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
        let mut needs_draw = false;
        if let Some((x, y)) = self.screen_coordinate(mouse_event.x(), mouse_event.y()) {
            needs_draw |= self.update_cursor(x, y);
            if self.mode == Mode::PointLight {
                match self.cursor {
                    Cursor::ExistingLight(_) => {}
                    _ => {
                        self.point_lights.push((x, y));
                        self.update_cursor(x, y);
                        needs_draw = true;
                    }
                }
            } else if self.mode == Mode::Erase {
                if let Cursor::ExistingLight(idx) = self.cursor {
                    self.point_lights.remove(idx);
                    needs_draw |= self.update_cursor(x, y);
                } else {
                    let (xb, yb) = block_coordinate(x, y);
                    needs_draw |= self.update_block_for_mode(xb, yb);
                }
            } else {
                let (xb, yb) = block_coordinate(x, y);
                needs_draw |= self.update_block_for_mode(xb, yb);
            }
        }

        if needs_draw {
            self.draw()?;
        }
        Ok(())
    }

    fn mouse_move(&mut self, mouse_event: MouseEvent) -> Result<(), Error> {
        let mut needs_draw = false;
        if let Some((x, y)) = self.screen_coordinate(mouse_event.x(), mouse_event.y()) {
            let mouse_down = (mouse_event.buttons() & 1) != 0;
            if self.mode == Mode::PointLight {
                if let Cursor::ExistingLight(idx) = self.cursor {
                    if mouse_down {
                        self.point_lights[idx].0 +=
                            mouse_event.movement_x() as f32 / self.width as f32;
                        self.point_lights[idx].1 -=
                            mouse_event.movement_y() as f32 / self.height as f32;
                        needs_draw = true;
                    }
                }
            }
            needs_draw |= self.update_cursor(x, y);
            if mouse_down {
                match self.cursor {
                    Cursor::None => {}
                    Cursor::Block(x, y) => {
                        needs_draw |= self.update_block_for_mode(x, y);
                    }
                    Cursor::ExistingLight(idx) => {
                        if self.mode == Mode::Erase {
                            self.point_lights.remove(idx);
                            self.update_cursor(x, y);
                            needs_draw = true;
                        }
                    }
                    Cursor::NewLight(_, _) => {}
                }
            }
        }

        if needs_draw {
            self.draw()?;
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
        self.context.clear_color(0.0, 0.0, 0.0, 1.0);
        self.context.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);

        self.renderer.start(1, 1, &[255, 255, 255])?;

        self.renderer.draw(
            (0.0, 0.0, 1.0, 1.0),
            BACKGROUND_TEX_COORDS,
            (1.0, 1.0, 1.0, 1.0),
        );

        for y in 0..BLOCK_COUNT.1 {
            for x in 0..BLOCK_COUNT.0 {
                let xc = x as f32 * BLOCK_SIZE.0;
                let yc = y as f32 * BLOCK_SIZE.1;

                match self.block_state[block_index(x, y)] {
                    BlockState::Foreground => {
                        let mut xxhash = XxHash::with_seed(17);
                        xxhash.write_u32(x);
                        xxhash.write_u32(y);
                        let block_tex = BLOCK_TEX_COORDS[(xxhash.finish() % 4) as usize];

                        self.renderer.draw(
                            (xc, yc, xc + BLOCK_SIZE.0, yc + BLOCK_SIZE.1),
                            block_tex,
                            (1.0, 1.0, 1.0, 1.0),
                        );

                        if y == BLOCK_COUNT.1 - 1
                            || self.block_state[block_index(x, y + 1)] == BlockState::Empty
                        {
                            let mut xxhash = XxHash::with_seed(23);
                            xxhash.write_u32(x);
                            xxhash.write_u32(y);
                            let grass_tex = GRASS_TEX_COORDS[(xxhash.finish() % 4) as usize];
                            self.renderer.draw(
                                (xc, yc, xc + BLOCK_SIZE.0, yc + BLOCK_SIZE.1),
                                grass_tex,
                                (1.0, 1.0, 1.0, 1.0),
                            );
                        }
                    }
                    BlockState::Background => {
                        let mut xxhash = XxHash::with_seed(11);
                        xxhash.write_u32(x);
                        xxhash.write_u32(y);
                        let block_tex = BLOCK_TEX_COORDS[(xxhash.finish() % 4) as usize];

                        self.renderer.draw(
                            (xc, yc, xc + BLOCK_SIZE.0, yc + BLOCK_SIZE.1),
                            block_tex,
                            (0.5, 0.5, 0.5, 1.0),
                        );
                    }
                    BlockState::Light => {
                        let mut xxhash = XxHash::with_seed(29);
                        xxhash.write_u32(x);
                        xxhash.write_u32(y);
                        let block_tex = LIGHT_BLOCK_TEX_COORDS[(xxhash.finish() % 4) as usize];

                        self.renderer.draw(
                            (xc, yc, xc + BLOCK_SIZE.0, yc + BLOCK_SIZE.1),
                            block_tex,
                            (1.0, 1.0, 1.0, 1.0),
                        );
                    }
                    BlockState::Empty => {}
                }
            }
        }

        for light in &self.point_lights {
            let light_rect = (
                light.0 - BLOCK_SIZE.0 / 2.0,
                light.1 - BLOCK_SIZE.1 / 2.0,
                light.0 + BLOCK_SIZE.0 / 2.0,
                light.1 + BLOCK_SIZE.1 / 2.0,
            );
            self.renderer
                .draw(light_rect, POINT_LIGHT_TEX_COORDS, (1.0, 1.0, 1.0, 1.0));
        }

        match self.cursor {
            Cursor::None => {}
            Cursor::Block(x, y) => {
                let xc = x as f32 * BLOCK_SIZE.0;
                let yc = y as f32 * BLOCK_SIZE.1;
                self.renderer.draw(
                    (xc, yc, xc + BLOCK_SIZE.0, yc + BLOCK_SIZE.1),
                    BLOCK_SELECT_TEX_COORDS,
                    (1.0, 1.0, 1.0, 1.0),
                );
            }
            Cursor::ExistingLight(idx) => {
                let light = self.point_lights[idx];
                let light_rect = (
                    light.0 - BLOCK_SIZE.0 / 2.0,
                    light.1 - BLOCK_SIZE.1 / 2.0,
                    light.0 + BLOCK_SIZE.0 / 2.0,
                    light.1 + BLOCK_SIZE.1 / 2.0,
                );
                self.renderer
                    .draw(light_rect, LIGHT_SELECT_TEX_COORDS, (1.0, 1.0, 1.0, 1.0));
            }
            Cursor::NewLight(x, y) => {
                let light_rect = (
                    x - BLOCK_SIZE.0 / 2.0,
                    y - BLOCK_SIZE.1 / 2.0,
                    x + BLOCK_SIZE.0 / 2.0,
                    y + BLOCK_SIZE.1 / 2.0,
                );
                self.renderer
                    .draw(light_rect, LIGHT_SELECT_TEX_COORDS, (1.0, 1.0, 1.0, 0.3));
            }
        }

        self.renderer.finish()?;

        Ok(())
    }

    fn update_cursor(&mut self, x: f32, y: f32) -> bool {
        let mut new_cursor = Cursor::None;
        let mut min_distance_squared = f32::MAX;
        if self.mode == Mode::PointLight || self.mode == Mode::Erase {
            for (i, light) in self.point_lights.iter().enumerate() {
                let cur_distance_squared = (light.0 - x).powf(2.0) + (light.1 - y).powf(2.0);
                if cur_distance_squared < LIGHT_SELECTION_RADIUS.powf(2.0)
                    && cur_distance_squared < min_distance_squared
                {
                    new_cursor = Cursor::ExistingLight(i);
                    min_distance_squared = cur_distance_squared;
                }
            }
        }

        if new_cursor == Cursor::None {
            if self.mode == Mode::PointLight {
                new_cursor = Cursor::NewLight(x, y);
            } else {
                let (xi, yi) = block_coordinate(x, y);
                new_cursor = Cursor::Block(xi, yi);
            }
        }

        if self.cursor != new_cursor {
            self.cursor = new_cursor;
            true
        } else {
            false
        }
    }

    fn update_block_for_mode(&mut self, x: u32, y: u32) -> bool {
        let bi = block_index(x, y);

        let new_block_state = match self.mode {
            Mode::ForegroundBlock => BlockState::Foreground,
            Mode::BackgroundBlock => BlockState::Background,
            Mode::LightBlock => BlockState::Light,
            Mode::PointLight => return false,
            Mode::Erase => BlockState::Empty,
        };

        if self.block_state[bi] != new_block_state {
            self.block_state[bi] = new_block_state;
            true
        } else {
            false
        }
    }

    fn screen_coordinate(&self, x: i32, y: i32) -> Option<(f32, f32)> {
        let x = x as f32 / self.width as f32;
        let y = 1.0 - y as f32 / self.height as f32;

        if x < 0.0 || x >= 1.0 || y < 0.0 || y >= 1.0 {
            None
        } else {
            Some((x, y))
        }
    }
}

fn block_coordinate(x: f32, y: f32) -> (u32, u32) {
    (
        (x * BLOCK_COUNT.0 as f32).floor() as u32,
        (y * BLOCK_COUNT.1 as f32).floor() as u32,
    )
}

fn block_index(x: u32, y: u32) -> usize {
    y as usize * BLOCK_COUNT.0 as usize + x as usize
}
