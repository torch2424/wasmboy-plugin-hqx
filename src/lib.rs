// Add the wasm-pack/wasm-bindgen crate
use wasm_bindgen::prelude::*;

// Add our hqx crate
// https://github.com/CryZe/wasmboy-rs/blob/master/hqx/src/lib.rs
use hqx::*;

// Our constants
const GB_WIDTH: usize = 160;
const GB_HEIGHT: usize = 144;
const HQX_SCALE: usize = 3;
const HQX_BUFFER_SIZE: usize = HQX_SCALE * HQX_SCALE * GB_WIDTH * GB_HEIGHT;

// Turn rgb image byte array, into u32
// https://github.com/CryZe/wasmboy-rs/blob/master/src/main.rs#L227
static mut INPUT_BUFFER: [u32; GB_WIDTH * GB_HEIGHT] = [0; GB_WIDTH * GB_HEIGHT];

// Define our memory that will house our final rgba array
static mut OUTPUT_BUFFER: [u8; HQX_BUFFER_SIZE * 4] = [0; HQX_BUFFER_SIZE * 4];

// Our pointer / memory management functions
#[wasm_bindgen]
pub fn get_input_buffer_pointer() -> *const u32 {
    let pointer: *const u32;
    unsafe {
        pointer = INPUT_BUFFER.as_ptr()
    }
    return pointer;
}

#[wasm_bindgen]
pub fn get_input_buffer_size() -> usize {
    let length: usize;
    unsafe {
        length = INPUT_BUFFER.len();
    }
    return length;
}

#[wasm_bindgen]
pub fn get_output_buffer_pointer() -> *const u8 {
    let pointer: *const u8;
    unsafe {
        pointer = OUTPUT_BUFFER.as_ptr()
    }
    return pointer;
}

#[wasm_bindgen]
pub fn get_output_buffer_size() -> usize {
    let length: usize;
    unsafe {
        length = OUTPUT_BUFFER.len()
    }
    return length;
}

#[wasm_bindgen]
pub fn set_output_buffer(index: usize, value: u8) -> u8 {
    let response: u8;
    unsafe {
        OUTPUT_BUFFER[index] = value;
        response = OUTPUT_BUFFER[index];
    }
    return response;
}

// Function to run our hqx conversion
#[wasm_bindgen]
pub fn hqx() {

    let mut hqx_buffer: [u32; HQX_BUFFER_SIZE] = [0; HQX_BUFFER_SIZE];

    // Finally lets pass in our vectors
    // https://github.com/CryZe/wasmboy-rs/blob/master/hqx/src/hq4x.rs#L704
    // https://github.com/CryZe/wasmboy-rs/blob/master/src/main.rs#L237
    unsafe {
        hq3x(&INPUT_BUFFER, &mut hqx_buffer, GB_WIDTH, GB_HEIGHT);
    }

    // Write our hqx buffer to our output buffer
    // Convert back to an rgba u8 buffer
    // Where every component r,g,b,a is it's own
    // index.
    let mut i = 0;
    while i < hqx_buffer.len() {
        // Get our r,g,b as u32
        // https://github.com/CryZe/wasmboy-rs/blob/master/hqx/src/common.rs#L3
        let alpha: u8 = ((hqx_buffer[i] >> 24) & 0xFF) as u8;
        let red: u8 = ((hqx_buffer[i] >> 16) & 0xFF) as u8;
        let green: u8 = ((hqx_buffer[i] >> 8) & 0xFF) as u8;
        let blue: u8 = (hqx_buffer[i] & 0xFF) as u8;

        let pixel_base = i * 4;
        unsafe {
            OUTPUT_BUFFER[pixel_base] = red;
            OUTPUT_BUFFER[pixel_base + 1] = green;
            OUTPUT_BUFFER[pixel_base + 2] = blue;
            OUTPUT_BUFFER[pixel_base + 3] = alpha;
        }

        // Increase to the next pixel (argba in u32)
        i += 1;
    }
}

