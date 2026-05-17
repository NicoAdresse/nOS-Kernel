#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u8)]

// COLOR ENUM -------------------------------------------------------------------------------------
pub enum Color {
    Black = 0x0,
    Blue = 0x1,
    Green = 0x2,
    Cyan = 0x3,
    Red = 0x4,
    Magenta = 0x5,
    Brown = 0x6,
    LightGray = 0x7,
    DarkGray = 0x8,
    LightBlue = 0x9,
    LightGreen = 0xA,
    LightCyan = 0xB,
    LightRed = 0xC,
    Pink = 0xD,
    Yellow = 0xE,
    White = 0xF,
}

// CONVERTING COLOR NAMES TO VALUES ---------------------------------------------------------------
impl Color {
    pub fn from_name(name: &str) -> Option<Self> {
        match name {
            "black" => Some(Self::Black),
            "blue" => Some(Self::Blue),
            "green" => Some(Self::Green),
            "cyan" => Some(Self::Cyan),
            "red" => Some(Self::Red),
            "magenta" => Some(Self::Magenta),
            "brown" => Some(Self::Brown),
            "lightgray" => Some(Self::LightGray),
            "darkgray" => Some(Self::DarkGray),
            "lightblue" => Some(Self::LightBlue),
            "lightgreen" => Some(Self::LightGreen),
            "lightcyan" => Some(Self::LightCyan),
            "lightred" => Some(Self::LightRed),
            "pink" => Some(Self::Pink),
            "yellow" => Some(Self::Yellow),
            "white" => Some(Self::White),
            _ => None,
        }
    }
}
