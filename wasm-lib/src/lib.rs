// #[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn test(v: f32) -> f32 {
    v * 2.0
}
