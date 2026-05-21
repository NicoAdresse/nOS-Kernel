#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum KeyEvent {
    Press(char),
    Release(char),
    Unknown,
}

/// Maps a raw, cleared PS/2 Set 1 scancode to its corresponding character.
fn scancode_to_char(scancode: u8) -> Option<char> {
    match scancode {
        0x1E => Some('a'),
        0x30 => Some('b'),
        0x2E => Some('c'),
        0x20 => Some('d'),
        0x12 => Some('e'),
        0x21 => Some('f'),
        0x22 => Some('g'),
        0x23 => Some('h'),
        0x17 => Some('i'),
        0x24 => Some('j'),
        0x25 => Some('k'),
        0x26 => Some('l'),
        0x32 => Some('m'),
        0x31 => Some('n'),
        0x18 => Some('o'),
        0x19 => Some('p'),
        0x10 => Some('q'),
        0x13 => Some('r'),
        0x1F => Some('s'),
        0x14 => Some('t'),
        0x16 => Some('u'),
        0x2F => Some('v'),
        0x11 => Some('w'),
        0x2D => Some('x'),
        0x15 => Some('y'),
        0x2C => Some('z'),
        0x33 => Some(','),
        0x34 => Some('.'),
        0x35 => Some('/'),
        0x37 => Some('*'),
        0x2B => Some('+'),
        0x39 => Some(' '),
        0x0E => Some('\x08'),
        0x1C => Some('\n'),
        _ => None,
    }
}

pub fn translate(sc: u8) -> KeyEvent {
    let is_release = (sc & 0x80) != 0;
    
    let base_scancode = sc & 0x7F;

    match scancode_to_char(base_scancode) {
        Some(c) if is_release => KeyEvent::Release(c),
        Some(c) => KeyEvent::Press(c),
        None => KeyEvent::Unknown,
    }
}