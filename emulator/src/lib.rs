use rust_chip8_opengl::processor::Processor;
use std::{cell::RefCell, panic, rc::Rc};
use wasm_bindgen::prelude::*;
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

    let canvas = get_canvas();

    *r_clone.borrow_mut() = Some(Closure::new(move || {
        log("Main loop called");

        i += 1;

        canvas.clear_color((i % 256) as f32 / 255.0, 0.0, 0.0, 1.0);
        canvas.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);

        get_window()
            .request_animation_frame(r.borrow().as_ref().unwrap().as_ref().unchecked_ref())
            .expect("Couldn't request animation frame");
    }));

    get_window()
        .request_animation_frame(r_clone.borrow().as_ref().unwrap().as_ref().unchecked_ref())
        .expect("Couldn't request animation frame");
}
