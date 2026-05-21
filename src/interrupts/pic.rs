use crate::keyboard::keyboard_input::{inb, outb};

pub unsafe fn pic_remap() {
    outb(0x20, 0x11);
    outb(0xA0, 0x11);

    outb(0x21, 0x20);
    outb(0xA1, 0x28);

    outb(0x21, 0x04);
    outb(0xA1, 0x02);

    outb(0x21, 0x01);
    outb(0xA1, 0x01);

    outb(0x21, 0xFD);
    outb(0xA1, 0xFF);
}

pub fn eoi(irq: u8) {
    unsafe {
        if irq >= 8 {
            outb(0xA0, 0x20);
        }
        outb(0x20, 0x20);
    }
}

pub fn unmask(irq: u8) {
    let port = if irq < 8 { 0x21 } else { 0xA1 };
    let bit = irq & 7;

    unsafe {
        let value = inb(port);
        outb(port, value & !(1 << bit));
    }
}
