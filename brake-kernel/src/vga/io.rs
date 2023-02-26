use core::fmt::{Write, self};

use spin::Mutex;

use crate::vga::{char::{VGACharAttribs, Color}, buffer::VGABuffer};

use super::buffer::Writer;

lazy_static::lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        column_pos: 0,
        attribs: VGACharAttribs::new(false, Color::Black, Color::LightGreen),
        buffer: unsafe { &mut *(0xb8000 as *mut VGABuffer) },
    });
}

pub fn _print(args: fmt::Arguments) {
    let mut writer = WRITER.lock();
    writer.write_fmt(args).ok();
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {{
        $crate::vga::io::_print($crate::format_args!($($arg)*));
    }};
}

#[macro_export]
macro_rules! println {
    () => {
        $crate::print!("\n")
    };
    ($($arg:tt)*) => {{
        $crate::print!("{}\n", $crate::format_args!($($arg)*));
    }};
}