use super::buffer::{HEIGHT, WIDTH, write_cell};
use super::Style;
use crate::scrolling::scroll_up;

// CURSOR TRACKING --------------------------------------------------------------------------------

static mut CURSOR_ROW: usize = 0;
static mut CURSOR_COL: usize = 0;

pub fn put_char(byte: u8, style: Style) {
    unsafe {
        match byte {
            b'\n' => {
                CURSOR_COL = 0;
                CURSOR_ROW += 1;
            }
            b'\r' => {
                CURSOR_COL = 0;
            }
            _ => {
                write_cell(CURSOR_ROW, CURSOR_COL, byte, style.fg, style.bg);
                CURSOR_COL += 1;

                if CURSOR_COL >= WIDTH {
                    CURSOR_COL = 0;
                    CURSOR_ROW += 1;
                }
            }
        }

        if CURSOR_ROW >= HEIGHT {
            scroll_up(style.bg);
            CURSOR_ROW = HEIGHT - 1;
        }
    }
}

// CURSOR CONTROL ---------------------------------------------------------------------------------

pub fn reset_cursor() {
    unsafe {
        CURSOR_ROW = 0;
        CURSOR_COL = 0;
    }
}

pub fn get_cursor() -> (usize, usize) {
    unsafe { (CURSOR_ROW, CURSOR_COL) }
}

pub fn set_cursor(row: usize, col: usize) {
    unsafe {
        CURSOR_ROW = row;
        CURSOR_COL = col;
    }
}