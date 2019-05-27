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

#[no_mangle]
unsafe fn get_input_buffer_pointer() -> *const u32 {
    return INPUT_BUFFER.as_ptr();
}

#[no_mangle]
unsafe fn get_input_buffer_size() -> usize {
    return INPUT_BUFFER.len();
}

#[no_mangle]
unsafe fn get_output_buffer_pointer() -> *const u8 {
    return OUTPUT_BUFFER.as_ptr();
}

#[no_mangle]
unsafe fn get_output_buffer_size() -> usize {
    return OUTPUT_BUFFER.len();
}

#[no_mangle]
unsafe fn set_output_buffer(index: usize, value: u8) -> u8 {
    OUTPUT_BUFFER[index] = value;
    return OUTPUT_BUFFER[index];
}

#[no_mangle]
unsafe fn hqx() {

    let mut hqx_buffer: [u32; HQX_BUFFER_SIZE] = [0; HQX_BUFFER_SIZE];

    // Finally lets pass in our vectors
    // https://github.com/CryZe/wasmboy-rs/blob/master/hqx/src/hq4x.rs#L704
    // https://github.com/CryZe/wasmboy-rs/blob/master/src/main.rs#L237
    hq3x(&INPUT_BUFFER, &mut hqx_buffer, GB_WIDTH, GB_HEIGHT);

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
        OUTPUT_BUFFER[pixel_base] = red;
        OUTPUT_BUFFER[pixel_base + 1] = green;
        OUTPUT_BUFFER[pixel_base + 2] = blue;
        OUTPUT_BUFFER[pixel_base + 3] = alpha;

        // Increase to the next pixel (argba in u32)
        i += 1;
    }
}

