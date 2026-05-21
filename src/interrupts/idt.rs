use core::mem::size_of;
use crate::keyboard::keyboard_input::InterruptStackFrame;

#[repr(C, packed)]
#[derive(Clone, Copy)]
struct IdtEntry {
    offset_low: u16,
    selector: u16,
    options: u16,
    offset_mid: u16,
    offset_high: u32,
    zero: u32,
}

impl IdtEntry {
    const fn missing() -> Self {
        Self {
            offset_low: 0,
            selector: 0,
            options: 0,
            offset_mid: 0,
            offset_high: 0,
            zero: 0,
        }
    }

    fn set_handler(&mut self, handler: unsafe extern "x86-interrupt" fn(&mut InterruptStackFrame)) {
        let addr = handler as u64;

        self.offset_low = addr as u16;
        self.selector = 0x18;
        self.options = 0x8E00;
        self.offset_mid = (addr >> 16) as u16;
        self.offset_high = (addr >> 32) as u32;
        self.zero = 0;
    }
}

#[repr(C, packed)]
struct IdtPtr {
    limit: u16,
    base: u64,
}

static mut IDT: [IdtEntry; 256] = [IdtEntry::missing(); 256];

unsafe extern "x86-interrupt" {
    unsafe fn irq1_handler(frame: &mut InterruptStackFrame);
}

pub unsafe fn init_idt() {
    IDT[0x21].set_handler(irq1_handler);

    let idt_ptr = IdtPtr {
        limit: (size_of::<[IdtEntry; 256]>() - 1) as u16,
        base: &IDT as *const _ as u64,
    };

    unsafe {
        core::arch::asm!(
            "lidt [{}]",
            in(reg) &idt_ptr,
            options(nostack, preserves_flags)
        );
    }
}
