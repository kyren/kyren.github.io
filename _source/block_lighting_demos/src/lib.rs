extern crate js_sys;
extern crate wasm_bindgen;
extern crate web_sys;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::Document;

pub mod demo;

#[wasm_bindgen]
pub fn init() {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();

    fn get_element<T: JsCast>(document: &Document, id: &str) -> T {
        document
            .get_element_by_id(id)
            .unwrap()
            .dyn_into::<T>()
            .unwrap()
    }

    let canvas = get_element(&document, "canvas");

    let mode_section = get_element(&document, "mode_controls");
    let solid_block_mode_radio = get_element(&document, "solid_block_mode");
    let light_block_mode_radio = get_element(&document, "light_block_mode");
    let point_light_mode_radio = get_element(&document, "point_light_mode");
    let erase_mode_radio = get_element(&document, "erase_mode");

    let angle_section = get_element(&document, "angle_controls");
    let angle_slider = get_element(&document, "angle");

    let pointiness_section = get_element(&document, "pointiness_controls");
    let pointiness_slider = get_element(&document, "pointiness");

    let spread_section = get_element(&document, "spread_controls");
    let spread_steps_slider = get_element(&document, "spread_steps");

    let clear_section = get_element(&document, "clear_controls");
    let clear_button = get_element(&document, "clear");

    let algorithm_section = get_element(&document, "algorithm_controls");
    let advance_button = get_element(&document, "advance");
    let reset_button = get_element(&document, "reset");

    demo::Demo::init(demo::DemoElements {
        canvas,

        mode_section,
        solid_block_mode_radio,
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
    });
}
