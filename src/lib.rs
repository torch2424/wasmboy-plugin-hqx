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
const HQX_BUFFER_SIZE: usize = 4 * 4 * GB_WIDTH * GB_HEIGHT;

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
pub fn hqx(image_data_array: &mut [u8]) -> [u8] {
    console_log!("Hello from {}!", "Rust");

    console_log!("Made it yo!");

    
    // TODO: Turn rgb byte array, unto u32
    // https://github.com/CryZe/wasmboy-rs/blob/master/src/main.rs#L227
    // TODO: Also, just realized for below. Just do it the easy way.
    // Iterate over the input, and cast to u32 manually :p
    let mut argb_buffer: [u32; GB_WIDTH * GB_HEIGHT];

    // Fill our u32 rgb buffer, with our byte
    // image data
    let mut i = 0;
    let alpha: u32 = 0xFF << 24;
    while i < image_data_array.len() {
        // Get our r,g,b as u32
        let red: u32 = (image_data_array[i]).into() << 16;
        let green: u32 = (image_data_array[i + 1]).into() << 8;
        let blue: u32 = (image_data_array[i + 2]).into();
        
        argb_buffer[i] = alpha + red + blue + green;

        // Increase to the next pixel (rgba)
        i += 4;
    }

    console_log!("Unwrapped!");

    // Create our destination array
    // This creates an array, and sets the length to be the gb width
    // Times our upscale factor.
    // https://github.com/CryZe/wasmboy-rs/blob/master/src/main.rs#L71
    let mut hqx_buffer: [u32; HQX_BUFFER_SIZE];

    console_log!("Made it here!");

    // Finally lets pass in our vectors
    // https://github.com/CryZe/wasmboy-rs/blob/master/hqx/src/hq4x.rs#L704
    // https://github.com/CryZe/wasmboy-rs/blob/master/src/main.rs#L237
    hq4x(&argb_buffer, &mut hqx_buffer, GB_WIDTH, GB_HEIGHT);

    console_log!("ran the hqx! Yay!");

    // Convert back to an rgba u8 buffer
    // Where every component r,g,b,a is it's own
    // index.
    let mut rgba_buffer: [u8; HQX_BUFFER_SIZE * 4];
    i = 0;
    while i < hqx_buffer.len() {
        // Get our r,g,b as u32
        let red: u8 = (hqx_buffer[i] >> 16 & 0xF) as u8;
        let green: u8 = (hqx_buffer[i] >> 8 & 0xF) as u8;
        let blue: u8 = (hqx_buffer[i] & 0xF) as u8;
        
        let pixelBase = i * 4;
        rgba_buffer[pixelBase] = red;
        rgba_buffer[pixelBase + 1] = green;
        rgba_buffer[pixelBase + 2] = blue;
        rgba_buffer[pixelBase + 3] = 255;

        // Increase to the next pixel (argba in u32)
        i += 1;
    }

    // Return a pointer to our rgba buffer

    return rgba_buffer;
}

