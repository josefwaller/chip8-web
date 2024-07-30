pub mod web_gl;

use rust_chip8_opengl::processor::Processor;
use std::{cell::RefCell, panic, rc::Rc};
use wasm_bindgen::prelude::*;
use web_gl::{
    buffer_data_f32, create_buffer_f32, create_buffer_i32, create_program, create_vertex_array,
};
use web_sys::{HtmlCanvasElement, HtmlInputElement, WebGl2RenderingContext, Window};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

pub fn get_window() -> Window {
    web_sys::window().expect("no global `window` exists")
}

pub fn get_canvas() -> WebGl2RenderingContext {
    let canvas = get_window()
        .document()
        .unwrap()
        .get_element_by_id("canvas")
        .unwrap();
    canvas
        .dyn_into::<HtmlCanvasElement>()
        .unwrap()
        .get_context("webgl2")
        .unwrap()
        .unwrap()
        .dyn_into::<WebGl2RenderingContext>()
        .unwrap()
}

#[wasm_bindgen]
pub fn start_main_loop(rom: &[u8]) {
    let mut p = Processor::new();
    p.load_program(rom);
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    log("Starting main loop");

    let r: Rc<RefCell<Option<Closure<dyn FnMut()>>>> = Rc::new(RefCell::new(None));
    let r_clone = r.clone();

    let context = get_canvas();

    let program = create_program(&context);
    // Generate vertices for screen
    const MIN_X: f32 = -1.0;
    const MAX_X: f32 = 1.0;
    const MIN_Y: f32 = 1.0;
    const MAX_Y: f32 = -1.0;
    const SCREEN_WIDTH: usize = 64;
    const SCREEN_HEIGHT: usize = 32;
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
    let colors_buffer = create_buffer_f32(&context, WebGl2RenderingContext::ARRAY_BUFFER, colors);
    // Point GLSL 'color' variable to the array
    context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&colors_buffer));
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

    let mut last_time = get_window()
        .performance()
        .expect("Can't get performance!")
        .now();
    let mut last_tick_time = last_time;

    *r_clone.borrow_mut() = Some(Closure::new(move || {
        let clock_speed = get_window()
            .document()
            .unwrap()
            .get_element_by_id("clock-speed")
            .unwrap()
            .dyn_into::<HtmlInputElement>()
            .unwrap()
            .value_as_number();
        let new_time = get_window()
            .performance()
            .expect("Can't get performance!")
            .now();

        if new_time - last_tick_time >= 1000.0 / 60.0 {
            p.on_tick();
            last_tick_time = new_time;
        }
        let dt = new_time - last_time;

        let n_steps = (dt / 1000.0 * clock_speed) as u32;
        for _ in 0..n_steps {
            p.step();
        }
        // Uncomment this for logging
        // log(format!(
        //     "DT: {:?}, FPS: {:?}, N Steps: {:?}, Clock Speed: {:?}",
        //     dt,
        //     1000.0 / dt,
        //     n_steps,
        //     clock_speed
        // )
        // .as_str());
        last_time = new_time;

        let mut new_colors = Vec::new();
        for x in 0..SCREEN_WIDTH {
            for y in 0..SCREEN_HEIGHT {
                for _ in 0..12 {
                    new_colors.push(if p.get_pixel_at(x as u8, y as u8) {
                        0.0
                    } else {
                        1.0
                    })
                }
            }
        }

        buffer_data_f32(
            &context,
            &colors_buffer,
            WebGl2RenderingContext::ARRAY_BUFFER,
            new_colors,
        );

        context.draw_elements_with_i32(
            WebGl2RenderingContext::TRIANGLES,
            indices_len as i32,
            WebGl2RenderingContext::UNSIGNED_INT,
            0,
        );
        get_window()
            .request_animation_frame(r.borrow().as_ref().unwrap().as_ref().unchecked_ref())
            .expect("Couldn't request animation frame");
    }));

    get_window()
        .request_animation_frame(r_clone.borrow().as_ref().unwrap().as_ref().unchecked_ref())
        .expect("Couldn't request animation frame");
}
