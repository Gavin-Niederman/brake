use pic8259::ChainedPics;
use spin::Mutex;

pub const PIC_1_OFFSET: u8 = 0x20;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 0x8;

pub static PICS: Mutex<ChainedPics> = Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });

pub unsafe fn init_pics() {
    PICS.lock().initialize();
    x86_64::instructions::interrupts::enable();
}

#[repr(u8)]
pub enum HardwareInterruptIndex {
    Timer = PIC_1_OFFSET,
    Keyboard,
}