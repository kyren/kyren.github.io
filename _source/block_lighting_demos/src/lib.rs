#![allow(dead_code)]

extern crate failure;
extern crate js_sys;
extern crate wasm_bindgen;
extern crate web_sys;

mod blocks;
mod demo;
mod glutil;
mod util;

use wasm_bindgen::prelude::*;

use self::util::{get_element, handle_error};

#[wasm_bindgen]
pub fn demo_init() {
    handle_error("initialization", || {
        let canvas = get_element("canvas")?;

        let mode_section = get_element("mode_controls")?;
        let solid_block_mode_radio = get_element("solid_block_mode")?;
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
        })?;

        Ok(())
    });
}
