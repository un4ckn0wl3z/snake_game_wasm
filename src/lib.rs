use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn greeting(name: &str) {
    print!("Hi there {}", name);
}