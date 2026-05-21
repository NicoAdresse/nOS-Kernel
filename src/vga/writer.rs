use super::Style;
use super::Color;
use super::cursor::put_char;
use super::buffer::{write_cell, HEIGHT, WIDTH};

// SCREEN CLEAR -----------------------------------------------------------------------------------

pub fn clear_screen(bg: Color) {
    for row in 0..HEIGHT {
        for col in 0..WIDTH {
            write_cell(row, col, b' ', Color::Black, bg);
        }
    }

    super::cursor::reset_cursor();
}


// CUSTOM PRINTS ----------------------------------------------------------------------------------
pub fn print(msg: &str, style: Style) {
    for byte in msg.bytes() {
        put_char(byte, style);
    }
}

pub fn println(msg: &str, style: Style) {
    print(msg, style);
    put_char(b'\n', style);
}

pub fn print_named(msg: &str, name: &str) {
    if let Some(style) = Style::from_name(name) {
        print(msg, style);
    }
}

pub fn println_named(msg: &str, name: &str) {
    if let Some(style) = Style::from_name(name) {
        println(msg, style);
    }
}

// USEFUL PRINTS ----------------------------------------------------------------------------------

pub fn print_ok(msg: &str) {
    print_named(msg, "lightgreen_black");
}

pub fn print_normal(msg: &str) {
    print_named(msg, "lightgray_black");
}

pub fn print_err(msg: &str) {
    print_named(msg, "lightred_black");
}

pub fn print_warn(msg: &str) {
    print_named(msg, "yellow_black");
}

pub fn print_info(msg: &str) {
    print_named(msg, "cyan_black");
}

pub fn println_ok(msg: &str) {
    println_named(msg, "lightgreen_black");
}

pub fn println_normal(msg: &str) {
    println_named(msg, "lightgray_black");
}

pub fn println_err(msg: &str) {
    println_named(msg, "lightred_black");
}

pub fn println_warn(msg: &str) {
    println_named(msg, "yellow_black");
}

pub fn println_info(msg: &str) {
    println_named(msg, "cyan_black");
}

// BOOT BANNER  -----------------------------------------------------------------------------------

pub fn print_boot_banner() {
    println_named("================================================================================", "lightcyan_black");
    println_named("|                                 nOS Kernel                                   |", "white_black");
    println_named("|                          Version 0.1.1 --- May 2026                          |", "white_black");
    println_named("|                          Developed by Nico Erdmann                           |", "white_black");
    println_named("================================================================================", "lightcyan_black");
}
