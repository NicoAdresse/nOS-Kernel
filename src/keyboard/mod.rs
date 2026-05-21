pub mod translate_scan;
pub mod vga_scan;
pub mod keyboard_input;

pub fn init() {
    keyboard_input::init();
}
