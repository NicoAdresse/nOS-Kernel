#![no_std]
#![no_main]

mod vga;
mod scrolling;

use core::panic::PanicInfo;

use crate::vga::{cursor::reset_cursor, writer::clear_screen};

/// Kernel Entry Point
#[no_mangle]
pub extern "C" fn _start() -> ! {
    reset_cursor();
    vga::print_boot_banner();
    // Enter a low-power execution halt state to prevent the CPU from executing unmapped memory
    loop {
        unsafe {
            core::arch::asm!("hlt", options(nomem, nostack, preserves_flags));
        }
    }
}

/// Fallback behavior if a core assertion or memory operation fails
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        unsafe {
            core::arch::asm!("hlt", options(nomem, nostack, preserves_flags));
        }
    }
}
