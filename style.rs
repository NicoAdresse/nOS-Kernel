use super::Color;

// STYLE STRUCT -----------------------------------------------------------------------------------
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Style {
    pub fg: Color,
    pub bg: Color,
}

// STYLE METHODS ----------------------------------------------------------------------------------
impl Style {
    pub const fn new(fg: Color, bg: Color) -> Self {
        Self { fg, bg }
    }

    pub const fn vga_attr(self) -> u8 {
        ((self.bg as u8) << 4) | (self.fg as u8)
    }

    pub fn from_name(name: &str) -> Option<Self> {
        let (fg_str, bg_str) = name.split_once('_')?;

        Some(Self {
            fg: Color::from_name(fg_str)?,
            bg: Color::from_name(bg_str)?,
        })
    }
}