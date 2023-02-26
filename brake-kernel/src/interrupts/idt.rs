use pc_keyboard::{layouts::Us104Key, HandleControl, Keyboard, ScancodeSet1};
use spin::Mutex;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

use crate::{gdt, interrupts::pics};

use super::pics::HardwareInterruptIndex;

static mut IDT: InterruptDescriptorTable = InterruptDescriptorTable::new();

static KEYBOARD: Mutex<Keyboard<Us104Key, ScancodeSet1>> = Mutex::new(Keyboard::new(
    ScancodeSet1::new(),
    Us104Key,
    HandleControl::Ignore,
));

pub unsafe fn init_idt() {
    IDT.double_fault
        .set_handler_fn(double_fault_handler)
        .set_stack_index(gdt::tss::DOUBLE_FAULT_IST_INDEX);
    IDT[HardwareInterruptIndex::Timer as u8 as _].set_handler_fn(timer_interrupt_handler);
    IDT[HardwareInterruptIndex::Keyboard as u8 as _].set_handler_fn(keyboard_interrupt_handler);
    IDT.load()
}

extern "x86-interrupt" fn double_fault_handler(
    stack_frame: InterruptStackFrame,
    err_code: u64,
) -> ! {
    panic!(
        "Double Fault! error code:{err_code}\n Stack Frame:\n{:#?}",
        stack_frame
    );
}

extern "x86-interrupt" fn timer_interrupt_handler(_stack_frame: InterruptStackFrame) {
    unsafe {
        pics::PICS
            .lock()
            .notify_end_of_interrupt(HardwareInterruptIndex::Timer as u8)
    }
}

extern "x86-interrupt" fn keyboard_interrupt_handler(_stack_frame: InterruptStackFrame) {
    let mut port = x86_64::instructions::port::Port::new(0x60);
    let scancode: u8 = unsafe { port.read() };

    let mut keyboard = KEYBOARD.lock();

    match keyboard.add_byte(scancode) {
        Ok(Some(event)) => {
            if let Some(key) = keyboard.process_keyevent(event) {
                match key {
                    pc_keyboard::DecodedKey::RawKey(raw) => print!("{:#?}", raw),
                    pc_keyboard::DecodedKey::Unicode(character) => print!("{character}"),
                }
            }
        }
        _ => {}
    }

    unsafe {
        pics::PICS
            .lock()
            .notify_end_of_interrupt(HardwareInterruptIndex::Keyboard as u8)
    }
}
