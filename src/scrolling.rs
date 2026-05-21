#![allow(dead_code)]

use crate::vga::buffer::{BUFFER_PTR, WIDTH, HEIGHT, write_cell};
use crate::vga::Color;

// SCROLLER ---------------------------------------------------------------------------------------
pub fn scroll_up(bg: Color) {
    let buffer = BUFFER_PTR as *mut u8;

    unsafe {
        let dst: *mut u8 = buffer;
        let src: *mut u8 = buffer.add(WIDTH * 2);
        let byte_count: usize = (HEIGHT - 1) * WIDTH * 2;

        core::ptr::copy(src, dst, byte_count);
    }

    let last_row = HEIGHT - 1;
    for col in 0..WIDTH {
        write_cell(last_row, col, b' ', Color::Black, bg);
    }
}
