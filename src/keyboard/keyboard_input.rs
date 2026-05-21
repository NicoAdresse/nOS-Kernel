use crate::keyboard::{translate_scan, vga_scan};
use core::arch::asm;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct InterruptStackFrame {
    pub instruction_pointer: u64,
    pub code_segment: u64,
    pub cpu_flags: u64,
    pub stack_pointer: u64,
    pub stack_segment: u64,
}

/// Reads a byte from the specified I/O port.
pub unsafe fn inb(port: u16) -> u8 {
    let value: u8;
    asm!(
        "in al, dx",
        out("al") value,
        in("dx") port,
        options(nomem, nostack, preserves_flags)
    );
    value
}

/// Writes a byte to the specified I/O port.
pub unsafe fn outb(port: u16, value: u8) {
    asm!(
        "out dx, al",
        in("dx") port,
        in("al") value,
        options(nomem, nostack, preserves_flags)
    );
}

/// Unmasks a specific IRQ line on the 8259 PIC.
pub fn pic_unmask(irq: u8) {
    let port = if irq < 8 { 0x21 } else { 0xA1 };
    let bit = irq % 8;

    unsafe {
        let value = inb(port);
        outb(port, value & !(1 << bit));
    }
}

/// Sends an End of Interrupt (EOI) signal to the 8259 PIC.
pub fn pic_eoi(irq: u8) {
    unsafe {
        if irq >= 8 {
            outb(0xA0, 0x20);
        }
        outb(0x20, 0x20);
    }
}

/// Initializes the keyboard driver by unmasking IRQ 1.
pub fn init() {
    pic_unmask(1);
}

/// Interrupt handler for IRQ 1 (Keyboard).
#[no_mangle]
pub extern "x86-interrupt" fn irq1_handler(_frame: &mut InterruptStackFrame) {
    let scancode = unsafe { inb(0x60) };

    let event = translate_scan::translate(scancode);
    vga_scan::handle_key(event);

    pic_eoi(1);
}