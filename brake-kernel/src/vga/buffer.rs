use core::fmt::Write;

use crate::vga::char::{VGAChar, VGACharAttribs};

const BUFFER_WIDTH: usize = 80;
const BUFFER_HEIGHT: usize = 25;

pub struct Writer {
    pub buffer: &'static mut VGABuffer,
    pub attribs: VGACharAttribs,
    pub column_pos: usize,
}

impl Write for Writer {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        s.bytes().for_each(|byte| self.write_byte(byte));

        Ok(())
    }
}

impl Writer {
    fn write_byte(&mut self, b: u8) {
        match b {
            b'\n' => self.new_line(),
            byte => {
                if self.column_pos > BUFFER_WIDTH {
                    self.new_line()
                }

                let row = BUFFER_HEIGHT - 1;
                let col = self.column_pos;

                self.buffer.chars[row][col] = VGAChar::new(byte, self.attribs);
                self.column_pos += 1;
            }
        }
    }

    fn new_line(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let character = self.buffer.chars[row][col];
                self.buffer.chars[row - 1][col] = character;
            }
        }
        self.clear_row(BUFFER_HEIGHT - 1);
        self.column_pos = 0;
    }

    fn clear_row(&mut self, row: usize) {
        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col] = VGAChar::new(b' ', self.attribs)
        }
    }
}

#[repr(transparent)]
pub struct VGABuffer {
    chars: [[VGAChar; BUFFER_WIDTH]; BUFFER_HEIGHT],
}