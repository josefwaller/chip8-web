use wasm_bindgen::prelude::{wasm_bindgen, JsCast};
use web_sys::{HtmlCanvasElement, WebGl2RenderingContext, Window};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

fn get_window() -> Window {
    web_sys::window().expect("no global `window` exists")
}

pub fn now() -> f64 {
    get_window()
        .performance()
        .expect("Can't get performance!")
        .now()
}

pub fn get_canvas() -> WebGl2RenderingContext {
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
