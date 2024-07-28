use rust_chip8_opengl::processor::Processor;
use std::{cell::RefCell, panic, rc::Rc};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

pub fn get_window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

#[wasm_bindgen]
pub fn start_main_loop() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    log("Starting main loop");

    let r: Rc<RefCell<Option<Closure<dyn FnMut()>>>> = Rc::new(RefCell::new(None));
    let r_clone = r.clone();

    let mut i = 0;

    *r_clone.borrow_mut() = Some(Closure::new(move || {
        log("Main loop called");

        i += 1;

        get_window()
            .request_animation_frame(r.borrow().as_ref().unwrap().as_ref().unchecked_ref())
            .expect("Couldn't request animation frame");
    }));

    get_window()
        .request_animation_frame(r_clone.borrow().as_ref().unwrap().as_ref().unchecked_ref())
        .expect("Couldn't request animation frame");
}
