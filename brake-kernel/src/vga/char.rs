#[derive(Clone, Copy)]
#[repr(C)]
pub struct VGAChar {
    character: u8,
    attribs: u8,
}

impl VGAChar {
    pub fn new(character: u8, attribs: VGACharAttribs) -> Self {
        Self {
            attribs: attribs.into(),
            character,
        }
    }
}

#[derive(Clone, Copy)]
pub struct VGACharAttribs {
    blinking: bool,    // bit 7
    background: Color, // bits 4-6
    foreground: Color, // bits 0-3
}

impl Into<u8> for VGACharAttribs {
    fn into(self) -> u8 {
        self.foreground as u8
            | (self.background as u8) << 4
            | if self.blinking { 0b1000_0000 } else { 0b0000_0000 }
    }
}

impl VGACharAttribs {
    pub fn new(blinking: bool, background: Color, foreground: Color) -> Self {
        Self {
            blinking,
            background,
            foreground,
        }
    }
}

#[allow(dead_code)]
#[derive(Clone, Copy)]
#[repr(u8)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    LightMagenta = 13,
    LightBrown = 14,
    White = 15,
}
