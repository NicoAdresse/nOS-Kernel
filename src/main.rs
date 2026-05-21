#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

mod interrupts;

mod vga;
mod scrolling;
pub mod keyboard;


use core::panic::PanicInfo;

/// Kernel Entry Point
#[no_mangle]
pub extern "C" fn _start() -> ! {
    interrupts::init();

    vga::print_boot_banner();
    vga::println_normal("Welcome to nOS! Write something: ");
    keyboard::init();

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
