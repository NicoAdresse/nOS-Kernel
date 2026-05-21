pub mod idt;
pub mod pic;

use self::idt::init_idt;
use self::pic::pic_remap;

pub fn init() {
    unsafe {
        init_idt();
        pic_remap();

        self::pic::unmask(2);
        self::pic::unmask(1);

        core::arch::asm!("sti");
    }
}
