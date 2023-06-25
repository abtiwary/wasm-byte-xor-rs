mod utils;

use wasm_bindgen::prelude::*;

use rand::Rng;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn generate_random_bytes(size: u32) -> Vec<u8> {
    let random_bytes: Vec<u8> = (0..size).map(|_| { rand::random::<u8>() }).collect();
    return random_bytes;
}

#[wasm_bindgen]
pub fn vec_xor (vec1: &mut [u8], vec2: &mut [u8]) {
    for (i, el) in vec1.iter_mut().enumerate() {
        *el ^= vec2[i];
    }
}
