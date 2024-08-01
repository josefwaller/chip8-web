pub mod web_gl;

use rust_chip8_opengl::processor::Processor;
use std::{array, cell::RefCell, panic, rc::Rc};
use wasm_bindgen::prelude::*;
use web_gl::{
    buffer_data_f32, create_buffer_f32, create_buffer_i32, create_program, create_vertex_array,
};
use web_sys::{
    Document, Event, HtmlCanvasElement, HtmlElement, HtmlInputElement, WebGl2RenderingContext,
    Window,
};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

fn get_window() -> Window {
    web_sys::window().expect("no global `window` exists")
}

fn get_document() -> Document {
    get_window().document().expect("Could not get document")
}

fn get_element_by_id(id: &str) -> HtmlElement {
    get_document()
        .get_element_by_id(id)
        .expect(format!("Could not find element with id #{}", id).as_str())
        .dyn_ref::<HtmlElement>()
        .expect(format!("Could not cast #{} to HtmlElement", id).as_str())
        .to_owned()
}

fn get_input_by_id(id: &str) -> HtmlInputElement {
    get_element_by_id(id)
        .dyn_into::<HtmlInputElement>()
        .expect(format!("Could not cast #{} into an HtmlInputElement", id).as_str())
}

fn add_input_event_listener<F: FnMut(&str) + 'static>(
    id: &'static str,
    event_type: &'static str,
    mut callback: F,
) {
    let f = Closure::<dyn FnMut(_)>::new(move |e: Event| {
        let value = e
            .current_target()
            .expect(format!("{} event on #{} had no current_target!", event_type, id).as_str())
            .dyn_into::<HtmlInputElement>()
            .expect(format!("Could not convert #{} to HtmlInputElement", id).as_str())
            .value()
            .as_str()
            .to_owned();
        callback(&value);
    });
    get_input_by_id(id)
        .add_event_listener_with_callback(event_type, f.as_ref().unchecked_ref())
        .expect(format!("Unable to add ann event listener on #{}", id).as_str());
    // Carefull!!!
    f.forget();
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

/*
 * This should contain everything that needs to be passed between JS and rust
 * Set using onChange events, then accessed in the main loop
 */
struct EmulatorInfo {
    p: Processor,
    inputs: [bool; 16],
    foreground_color: [f32; 3],
    background_color: [f32; 3],
}

#[wasm_bindgen]
pub fn start_main_loop(rom: &[u8]) {
    let emu = Rc::new(RefCell::new(EmulatorInfo {
        p: Processor::new(),
        inputs: [false; 16],
        foreground_color: [1.0, 1.0, 1.0],
        background_color: [0.0, 0.0, 0.0],
    }));
    emu.borrow_mut().p.load_program(rom);
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

    let emu_clone = emu.clone();
    add_input_event_listener("foreground-color", "change", move |v| {
        emu_clone.borrow_mut().foreground_color = hex_string_to_color(v);
    });
    let emu_clone = emu.clone();
    add_input_event_listener("background-color", "change", move |v| {
        emu_clone.borrow_mut().background_color = hex_string_to_color(v);
    });

    for i in 0..16 {
        let emu_clone = emu.clone();
        let on_click = Closure::<dyn FnMut()>::new(move || {
            emu_clone.borrow_mut().inputs[i] = true;
        });
        let id = format!("input_{}", i).as_str().to_owned();

        get_element_by_id(&id)
            .add_event_listener_with_callback("mousedown", on_click.as_ref().unchecked_ref())
            .expect("Unable to add mousedown event listener");
        on_click.forget();

        let emu_clone = emu.clone();
        let on_release = Closure::<dyn FnMut()>::new(move || {
            emu_clone.borrow_mut().inputs[i] = false;
            emu_clone.borrow_mut().p.on_key_release(i as u8);
        });
        get_element_by_id(&id)
            .add_event_listener_with_callback("mouseup", on_release.as_ref().unchecked_ref())
            .expect("Unable to add mouseup event listener");
        on_release.forget();
    }

    let mut last_time = get_window()
        .performance()
        .expect("Can't get performance!")
        .now();
    let mut last_tick_time = last_time;

    let emu_clone = emu.clone();
    *r_clone.borrow_mut() = Some(Closure::new(move || {
        let mut e = emu_clone.borrow_mut();
        let inputs = e.inputs;
        let foreground_color = e.foreground_color;
        let background_color = e.background_color;
        let p = &mut e.p;
        let clock_speed = get_document()
            .get_element_by_id("clock-speed")
            .expect("Could not get clock-speed")
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

        p.update_inputs(inputs);

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
                for _ in 0..4 {
                    new_colors.extend(if p.get_pixel_at(x as u8, y as u8) {
                        foreground_color
                    } else {
                        background_color
                    });
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
