pub mod color;
pub mod style;
pub mod buffer;
pub mod cursor;
pub mod writer;

pub use self::color::Color;
pub use self::style::Style;

#[allow(unused_imports)]
pub use self::buffer::{WIDTH, HEIGHT};

#[allow(unused_imports)]
pub use self::writer::{print, println, print_named, println_named, print_err, print_ok, print_normal, 
                        print_info, print_warn, println_ok, println_err, println_normal, println_info, 
                        println_warn, print_boot_banner};
