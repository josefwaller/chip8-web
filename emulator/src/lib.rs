pub mod web_gl;

use js_sys::Array;
use rust_chip8_opengl::processor::Processor;
use std::{array, panic};
use wasm_bindgen::prelude::*;
use web_gl::{
    buffer_data_f32, create_buffer_f32, create_buffer_i32, create_program, create_vertex_array,
};
use web_sys::{HtmlCanvasElement, WebGl2RenderingContext, WebGlBuffer, Window};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

fn get_window() -> Window {
    web_sys::window().expect("no global `window` exists")
}

fn now() -> f64 {
    get_window()
        .performance()
        .expect("Can't get performance!")
        .now()
}

fn hex_string_to_color(hex: &str) -> [f32; 3] {
    return array::from_fn(|i| {
        u32::from_str_radix(&hex[(2 * i + 1)..(2 * i + 3)], 16)
            .expect(format!("Invalid string passed to hex_string_to_color: '{}'", hex).as_str())
            as f32
            / 0xFF as f32
    });
}

fn get_canvas() -> WebGl2RenderingContext {
    let canvas = get_window()
        .document()
        .expect("Could not get document")
        .get_element_by_id("canvas")
        .expect("Could not get canvas");
    canvas
        .dyn_into::<HtmlCanvasElement>()
        .expect("Could not transform canvas")
        .get_context("webgl2")
        .expect("Could not get webgl2 context")
        .unwrap()
        .dyn_into::<WebGl2RenderingContext>()
        .expect("Could not cast into rendering context")
}

const SCREEN_WIDTH: usize = 64;
const SCREEN_HEIGHT: usize = 32;

struct RenderInfo {
    context: WebGl2RenderingContext,
    colors_buf: WebGlBuffer,
    indices_len: usize,
}
fn init_wegl() -> RenderInfo {
    let context = get_canvas();

    let program = create_program(&context);
    // Generate vertices for screen
    const MIN_X: f32 = -1.0;
    const MAX_X: f32 = 1.0;
    const MIN_Y: f32 = 1.0;
    const MAX_Y: f32 = -1.0;
    const PIXEL_WIDTH: f32 = (MAX_X - MIN_X) / SCREEN_WIDTH as f32;
    const PIXEL_HEIGHT: f32 = (MAX_Y - MIN_Y) / SCREEN_HEIGHT as f32;

    let vertices: Vec<f32> = (0..SCREEN_WIDTH)
        .map(|x| {
            // move here just means copy x
            (0..SCREEN_HEIGHT).map(move |y| {
                // Generate triangles for the pixel at (x, y)
                (0..2)
                    .map(move |x_off| {
                        (0..2)
                            .map(move |y_off| {
                                vec![
                                    MIN_X + PIXEL_WIDTH * (x + x_off) as f32,
                                    MIN_Y + PIXEL_HEIGHT * (y + y_off) as f32,
                                    0.0,
                                ]
                            })
                            .flatten()
                    })
                    .flatten()
            })
        })
        .flatten()
        .flatten()
        .collect();
    // Send vertices to GPU
    create_vertex_array(&context, vertices);

    let indices: Vec<i32> = (0..SCREEN_WIDTH)
        .map(|x| {
            (0..SCREEN_HEIGHT).map(move |y| {
                let i = (x * SCREEN_HEIGHT + y) * 4;
                [0, 1, 2, 1, 2, 3]
                    .map(|j| (j + i) as i32)
                    .into_iter()
                    .collect::<Vec<i32>>()
            })
        })
        .flatten()
        .flatten()
        .collect();
    let indices_len = indices.len();
    // Send indices to GPU
    let indices_buffer = create_buffer_i32(
        &context,
        WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER,
        indices,
    );
    context.bind_buffer(
        WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER,
        Some(&indices_buffer),
    );
    let position_attribute_location = context.get_attrib_location(&program, "position");
    context.vertex_attrib_pointer_with_i32(0 as u32, 3, WebGl2RenderingContext::FLOAT, false, 0, 0);
    context.enable_vertex_attrib_array(position_attribute_location as u32);

    // Send colours to GPU
    let colors = (0..(4 * SCREEN_WIDTH * SCREEN_HEIGHT))
        .map(|i| {
            let c = ((i / 6) % 2) as f32;
            vec![c, c, c]
        })
        .flatten()
        .collect();
    // Create buffer
    let colors_buf = create_buffer_f32(&context, WebGl2RenderingContext::ARRAY_BUFFER, colors);
    // Point GLSL 'color' variable to the array
    context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&colors_buf));
    let cidx = context.get_attrib_location(&program, "color");
    context.vertex_attrib_pointer_with_i32(
        cidx as u32,
        3,
        WebGl2RenderingContext::FLOAT,
        false,
        0,
        0,
    );
    context.enable_vertex_attrib_array(cidx as u32);

    RenderInfo {
        context,
        colors_buf,
        indices_len,
    }
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
}

#[wasm_bindgen]
impl EmulatorInfo {
    pub fn update(&mut self, inputs: &Array, clock_speed: f64, last_key_up: &JsValue) {
        //log(format!("{:?}", inputs).as_str());
        let i: [bool; 16] = inputs
            .iter()
            .map(|v| v.as_bool().unwrap())
            .collect::<Vec<bool>>()
            .try_into()
            .expect("Error collecting inputs");
        self.p.update_inputs(i);

        match last_key_up.as_f64() {
            Some(i) => self.p.on_key_release(i as u8),
            None => {}
        }

        let t = now();
        if t - self.lct > 1000.0 / 60.0 {
            self.p.on_tick();
            self.lct = t;
        }
        let dt = t - self.lt;
        let n_steps = (dt / 1000.0 * clock_speed) as u32;
        for _ in 0..n_steps {
            self.p.step();
        }
        self.lt = t;
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
}
#[wasm_bindgen]
pub fn setup(rom: &[u8]) -> EmulatorInfo {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    let mut emu = EmulatorInfo {
        p: Processor::new(),
        lt: now(),
        lct: now(),
        ri: init_wegl(),
    };
    emu.p.load_program(rom);

    emu
}
