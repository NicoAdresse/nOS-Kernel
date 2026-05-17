use super::Color;

pub const WIDTH: usize = 80;
pub const HEIGHT: usize = 25;
pub const BUFFER_PTR: usize = 0xb8000;

// LOW-LEVEL BUFFER ACCESS ------------------------------------------------------------------------
pub fn write_cell(row: usize, col: usize, byte: u8, fg: Color, bg: Color) {
    let offset = (row * WIDTH + col) * 2;

    unsafe {
        let ptr = BUFFER_PTR as *mut u8;
        core::ptr::write_volatile(ptr.add(offset), byte);
        core::ptr::write_volatile(ptr.add(offset + 1), ((bg as u8) << 4) | (fg as u8));
    }
}