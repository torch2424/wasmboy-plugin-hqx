mod utils;

// Using serde to handle JS Objects in rust.
// https://rustwasm.github.io/docs/wasm-bindgen/reference/arbitrary-data-with-serde.html

// Add a data structure serializer / deserializer.
#[macro_use]
extern crate serde_derive;

// Add a JSON Parsing crate, to work off of serde
extern crate serde_json;

// Add our hqx crate
// https://github.com/CryZe/wasmboy-rs/blob/master/hqx/src/lib.rs
use hqx::*;

// Add wasm_bindgen to do all the wasm<->JS interop for us
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

const GB_WIDTH: usize = 160;
const GB_HEIGHT: usize = 144;

// Adding console log support
// https://github.com/rustwasm/wasm-bindgen/blob/master/examples/console_log/src/lib.rs
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}
macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

// How to pass around memory
// https://github.com/rustwasm/wasm-bindgen/issues/964
// https://rustwasm.github.io/docs/wasm-bindgen/reference/types/number-slices.html
#[wasm_bindgen]
pub fn hqx(image_data_array: &mut [u8]) -> [u32] {
    console_log!("Hello from {}!", "Rust");

    console_log!("Made it yo!");

    
    // TODO: Turn rgb byte array, unto u32
    // https://github.com/CryZe/wasmboy-rs/blob/master/src/main.rs#L227
    // TODO: Also, just realized for below. Just do it the easy way.
    // Iterate over the input, and cast to u32 manually :p
    let mut buffer = [u32; GB_WIDTH * GB_HEIGHT];

    // Get our passed u8IntArray as a u32 array for hqx
    // Need to iterate over the array, map to case all values to u32
    // Then collect to add to the return value
    // https://stackoverflow.com/questions/16755181/type-casting-arrays-vectors-in-rust
    let image_data_casted: [u32] = image_data_array
        .iter()
        .map(|&e| e as u32)
        .collect();

    console_log!("Unwrapped!");

    // Create our destination array
    // This creates an array, and sets the length to be the gb width
    // Times our upscale factor.
    // https://github.com/CryZe/wasmboy-rs/blob/master/src/main.rs#L71
    let mut hqx_vector = [u32; (4 * 4 * GB_WIDTH * GB_HEIGHT)];

    console_log!("Made it here!");

    // Finally lets pass in our vectors
    // https://github.com/CryZe/wasmboy-rs/blob/master/hqx/src/hq4x.rs#L704
    // https://github.com/CryZe/wasmboy-rs/blob/master/src/main.rs#L237
    hq4x(&image_data_casted, &mut hqx_vector, GB_WIDTH, GB_HEIGHT);

    console_log!("ran the hqx! Yay!");

    // Return our hqx vector
    return hqx_vector;
}

