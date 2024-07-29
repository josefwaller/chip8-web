pub mod web_gl;

use rust_chip8_opengl::processor::Processor;
use std::{cell::RefCell, panic, rc::Rc};
use wasm_bindgen::prelude::*;
use web_gl::{
    buffer_data_f32, create_buffer_f32, create_buffer_i32, create_program, create_vertex_array,
};
use web_sys::{HtmlCanvasElement, WebGl2RenderingContext, Window};

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
pub fn start_main_loop() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    log("Starting main loop");

    let r: Rc<RefCell<Option<Closure<dyn FnMut()>>>> = Rc::new(RefCell::new(None));
    let r_clone = r.clone();

    let mut i = 0;

    let context = get_canvas();

    let program = create_program(&context);
    // Generate vertices for screen
    const MIN_X: f32 = -1.0;
    const MAX_X: f32 = 1.0;
    const MIN_Y: f32 = -1.0;
    const MAX_Y: f32 = 1.0;
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
    context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&colors_buffer));
    // Point GLSL 'color' variable to the array
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

    log(format!("{} indices", indices_len).as_str());
    *r_clone.borrow_mut() = Some(Closure::new(move || {
        log("Main loop called");

        i += 1;
        let new_colors: Vec<f32> = (0..(4 * SCREEN_WIDTH * SCREEN_HEIGHT))
            .map(|_| {
                let c = (i % 255) as f32 / 256.0;
                vec![c, c, c]
            })
            .flatten()
            .collect();
        buffer_data_f32(
            &context,
            &colors_buffer,
            WebGl2RenderingContext::ARRAY_BUFFER,
            new_colors,
        );

        context.clear_color((i % 256) as f32 / 255.0, 0.0, 0.0, 1.0);
        context.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);

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
