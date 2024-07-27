use rust_chip8_opengl::processor::Processor;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn add(a: u32, b: u32) -> f64 {
    let mut p = Processor::new();
    p.execute(0x6000 | (a & 0xFF) as u16);
    p.execute(0x6100 | (b & 0xFF) as u16);
    p.execute(0x8014);
    return p.get_register_value(0x0) as f64;
}
