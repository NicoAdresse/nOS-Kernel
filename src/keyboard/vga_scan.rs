use crate::vga;
use crate::vga::print_normal;
use crate::vga::cursor;
use crate::keyboard::translate_scan::KeyEvent;
use crate::vga::println_normal;

pub fn handle_key(event: KeyEvent) {
    if let KeyEvent::Press(c) = event {
        match c {
            '\x08' => { handle_backspace(); }
            '\n' => { println_normal(""); }
            _ => {
                let mut buf = [0u8; 1];
                buf[0] = c as u8;
                let s = core::str::from_utf8(&buf).unwrap_or("?");
                print_normal(s);
            }
        }
    }
}


fn handle_backspace() {
    let (row, col) = cursor::get_cursor();

    if col > 0 {
        let target_col: usize = col - 1;
        cursor::set_cursor(row, target_col);
        vga::buffer::write_cell(row, target_col, 0x20,
             vga::Color::White, vga::Color::Black);
    }
}

fn handle_dangerous_backspace() {
    let (row, col) = cursor::get_cursor();

    if col > 0 {
        let target_col: usize = col - 1;
        cursor::set_cursor(row, target_col);

        vga::buffer::write_cell(row, target_col, 0x20,
            vga::Color::White, vga::Color::Black);

    } else if row > 0 {
        let prev_row: usize = row - 1;
        let last_col: usize = vga::WIDTH - 1;

        cursor::set_cursor(prev_row, last_col);
        vga::buffer::write_cell(prev_row, last_col, 0x20,
            vga::Color::White, vga::Color::Black);
    }
}
