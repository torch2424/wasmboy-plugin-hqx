// Add our hqx crate
// https://github.com/CryZe/wasmboy-rs/blob/master/hqx/src/lib.rs
use hqx::*;

// Our constants
const GB_WIDTH: usize = 160;
const GB_HEIGHT: usize = 144;
const HQX_SCALE: usize = 3;
const HQX_BUFFER_SIZE: usize = HQX_SCALE * HQX_SCALE * GB_WIDTH * GB_HEIGHT;

// Define our memory that will house our final rgba array
static mut RGBA_BUFFER: [u8; HQX_BUFFER_SIZE * 4] = [0; HQX_BUFFER_SIZE * 4];

#[no_mangle]
unsafe fn get_output_buffer_pointer() -> *const u8 {
    return RGBA_BUFFER.as_ptr();
}

#[no_mangle]
unsafe fn get_output_index(index: usize) -> u8 {
    return RGBA_BUFFER[index];
}

#[no_mangle]
unsafe fn set_output_index(index: usize, value: u8) -> u8 {
    RGBA_BUFFER[index] = value;
    return RGBA_BUFFER[index];
}

#[no_mangle]
fn hqx() -> u8 {
    return 125;
}

