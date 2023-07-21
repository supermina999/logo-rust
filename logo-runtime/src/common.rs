use wasm_bindgen::prelude::*;

#[derive(Copy, Clone, PartialEq, Debug)]
#[wasm_bindgen]
pub struct Pos {
    pub x: f64,
    pub y: f64
}
