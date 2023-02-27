#![allow(dead_code)]

use wasm_bindgen::prelude::*;
use wee_alloc::WeeAlloc;

// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC:  WeeAlloc = WeeAlloc::INIT;


#[wasm_bindgen]
pub fn greeting(name: &str) {
    alert(name);
}

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
struct World {
     width: usize
}

#[wasm_bindgen]
impl World {
    pub fn new () -> Self {
        Self { width: 8 }
    }

    pub fn width(&self) -> usize {
        self.width
    }
}