use wasm_bindgen::prelude::*;

extern crate wee_alloc;

// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


#[wasm_bindgen]
pub fn greeting(name: &str) {
    alert(name);
}

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}