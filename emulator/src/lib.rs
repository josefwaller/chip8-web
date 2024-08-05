pub mod dom;
pub mod web_gl;

use dom::{log, now};
use js_sys::Array;
use rust_chip8_opengl::processor::Processor;
use std::{array, panic};
use wasm_bindgen::prelude::*;
use web_gl::{buffer_data_f32, init_wegl, RenderInfo, SCREEN_HEIGHT, SCREEN_WIDTH};
use web_sys::WebGl2RenderingContext;

fn hex_string_to_color(hex: &str) -> [f32; 3] {
    return array::from_fn(|i| {
        u32::from_str_radix(&hex[(2 * i + 1)..(2 * i + 3)], 16)
            .expect(format!("Invalid string passed to hex_string_to_color: '{}'", hex).as_str())
            as f32
            / 0xFF as f32
    });
}

/*
 * This should contain everything that needs to be passed between JS and rust
 * Set using onChange events, then accessed in the main loop
 */
#[wasm_bindgen]
pub struct EmulatorInfo {
    p: Processor,
    // Last time, used for frame rate/number of steps to advance
    lt: f64,
    // Last clock time, used for the sound and delay registers
    lct: f64,
    ri: RenderInfo,
    // bool that is true if ST > 0, i.e. if the browser should make a sound
    sound: bool,
}

#[wasm_bindgen]
impl EmulatorInfo {
    #[wasm_bindgen(constructor)]
    pub fn new(rom: &[u8]) -> EmulatorInfo {
        panic::set_hook(Box::new(console_error_panic_hook::hook));
        let mut emu = EmulatorInfo {
            p: Processor::new(),
            lt: now(),
            lct: now(),
            ri: init_wegl(),
            sound: false,
        };
        emu.p.load_program(rom);

        emu
    }
    pub fn update(&mut self, inputs: &Array, clock_speed: f64) {
        let i: [bool; 16] = inputs
            .iter()
            .map(|v| v.as_bool().unwrap())
            .collect::<Vec<bool>>()
            .try_into()
            .expect("Error collecting inputs");
        self.p.update_inputs(i);

        let t = now();
        if t - self.lct > 1000.0 / 60.0 {
            self.p.on_tick();
            self.lct = t;
        }
        let dt = t - self.lt;
        let n_steps = (dt / 1000.0 * clock_speed) as u32;
        for _ in 0..n_steps {
            self.p.step().ok();
        }
        self.lt = t;
        self.sound = self.p.get_st() > 0;
    }
    pub fn render(&self, foreground_color_str: &str, background_color_str: &str) {
        let foreground_color = hex_string_to_color(foreground_color_str);
        let background_color = hex_string_to_color(background_color_str);
        let mut new_colors = Vec::new();
        for x in 0..SCREEN_WIDTH {
            for y in 0..SCREEN_HEIGHT {
                for _ in 0..4 {
                    new_colors.extend(if self.p.get_pixel_at(x as u8, y as u8) {
                        foreground_color
                    } else {
                        background_color
                    });
                }
            }
        }
        buffer_data_f32(
            &self.ri.context,
            &self.ri.colors_buf,
            WebGl2RenderingContext::ARRAY_BUFFER,
            new_colors,
        );

        self.ri.context.draw_elements_with_i32(
            WebGl2RenderingContext::TRIANGLES,
            self.ri.indices_len as i32,
            WebGl2RenderingContext::UNSIGNED_INT,
            0,
        );
    }

    #[wasm_bindgen(js_name = getSound)]
    pub fn get_sound(&self) -> bool {
        return self.sound;
    }
}
