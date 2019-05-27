mod utils;

// Add our hqx crate
// https://github.com/CryZe/wasmboy-rs/blob/master/hqx/src/lib.rs
use hqx::*;

// Add wasm_bindgen to do all the wasm<->JS interop for us
extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

const GB_WIDTH: usize = 160;
const GB_HEIGHT: usize = 144;
const HQX_SCALE: usize = 3;
const HQX_BUFFER_SIZE: usize = HQX_SCALE * HQX_SCALE * GB_WIDTH * GB_HEIGHT;

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

// How to pass around by memory:
// https://github.com/rustwasm/wasm-bindgen/issues/111#issuecomment-455268735

/*
// Define our Buffers
const ARGB_BUFFER: [u32; GB_WIDTH * GB_HEIGHT] = [0; GB_WIDTH * GB_HEIGHT];
const ARGB_BUFFER_POINTER: *const u32 = ARGB_BUFFER.as_ptr();

// Create our destination array for the hqx function
// This creates an array, and sets the length to be the gb width
// Times our upscale factor.
// https://github.com/CryZe/wasmboy-rs/blob/master/src/main.rs#L71
const HQX_BUFFER: [u32; HQX_BUFFER_SIZE] = [0; HQX_BUFFER_SIZE];

*/

// Our final RGBA image data arrays passed back to js
const RGBA_BUFFER: [u8; GB_WIDTH * GB_HEIGHT * 3 * 4] = [0; GB_WIDTH * GB_HEIGHT * 3 * 4];
const RGBA_BUFFER_POINTER: *const u8 = RGBA_BUFFER.as_ptr();

/*
#[wasm_bindgen]
pub fn get_pointer_to_32_bit_input_buffer() -> *const u32 {
    return ARGB_BUFFER_POINTER;
}
*/

#[wasm_bindgen]
pub fn get_pointer_to_8_bit_output_buffer() -> *const u8 {
    return RGBA_BUFFER_POINTER;
}

/*
// TODO: Write function to return index 0 of a buffer.
// This is to test if pointers are woking
#[wasm_bindgen]
pub fn get_index(index: usize) -> u32 {
    return ARGB_BUFFER[index];
}
*/


// How to pass around memory
// https://github.com/rustwasm/wasm-bindgen/issues/964
// https://rustwasm.github.io/docs/wasm-bindgen/reference/types/number-slices.html
// https://github.com/rustwasm/wasm-bindgen/issues/111#issuecomment-455329490
#[wasm_bindgen]
pub fn hqx() -> Vec<u8> {  

    /*
    console_log!("wasm argb {}", ARGB_BUFFER[0]);
    console_log!("wasm hqx {}", HQX_BUFFER[0]);
    console_log!("wasm rgba {}", RGBA_BUFFER[0]);

    // Finally lets pass in our vectors
    // https://github.com/CryZe/wasmboy-rs/blob/master/hqx/src/hq4x.rs#L704
    // https://github.com/CryZe/wasmboy-rs/blob/master/src/main.rs#L237
    // hq3x(&ARGB_BUFFER, &mut HQX_BUFFER, GB_WIDTH, GB_HEIGHT);

    console_log!("wasm argb {}", ARGB_BUFFER[0]);
    console_log!("wasm hqx 0 {}", HQX_BUFFER[0]);
    console_log!("wasm hqx 1 {}", HQX_BUFFER[1]);
    console_log!("wasm hqx & {}", &HQX_BUFFER[0]);
    */

    /*
    // Convert back to an rgba u8 buffer
    // Where every component r,g,b,a is it's own
    // index.
    // Define our memory that will house our final rgba array
    let mut i = 0;
    while i < HQX_BUFFER.len() {
        // Get our r,g,b as u32
        let red: u8 = (HQX_BUFFER[i] >> 16 & 0xF) as u8;
        let green: u8 = (HQX_BUFFER[i] >> 8 & 0xF) as u8;
        let blue: u8 = (HQX_BUFFER[i] & 0xF) as u8;
        
        let pixel_base = i * 4;
        RGBA_BUFFER[pixel_base] = red;
        RGBA_BUFFER[pixel_base + 1] = green;
        RGBA_BUFFER[pixel_base + 2] = blue;
        RGBA_BUFFER[pixel_base + 3] = 255;

        // Increase to the next pixel (argba in u32)
        i += 1;
    }
    */

    let mut i = 0;
    while i < RGBA_BUFFER.len() {
        let pixel_base = i;
        RGBA_BUFFER[pixel_base] = 55;
        RGBA_BUFFER[pixel_base + 1] = 55;
        RGBA_BUFFER[pixel_base + 2] = 55;
        RGBA_BUFFER[pixel_base + 3] = 255;

        // Increase to the next pixel (argba in u32)
        i += 4;
    }

    console_log!("wasm rgba 0 {}", RGBA_BUFFER[0]);
    console_log!("wasm rgba 3 {}", RGBA_BUFFER[3]);

    return RGBA_BUFFER.to_vec();
}

