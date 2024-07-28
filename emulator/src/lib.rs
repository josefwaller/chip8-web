use rust_chip8_opengl::processor::Processor;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub struct State {
    p: Processor,
}

#[wasm_bindgen]
impl State {
    pub fn set_inputs(&mut self, inputs: &[i32]) {
        self.p
            .update_inputs(core::array::from_fn(|i| inputs[i] == 1));
    }
}

#[wasm_bindgen]
pub fn init_state() -> State {
    return State {
        p: Processor::new(),
    };
}

#[wasm_bindgen]
pub fn add(a: u32, b: u32) -> f64 {
    let mut p = Processor::new();
    p.execute(0x6000 | (a & 0xFF) as u16);
    p.execute(0x6100 | (b & 0xFF) as u16);
    p.execute(0x8014);
    return p.get_register_value(0x0) as f64;
}

#[wasm_bindgen]
pub fn loop_forever(mut s: State) -> State {
    if s.p.get_input_state(0xA) {
        log("A button pressed!");
    }
    return s;
}
