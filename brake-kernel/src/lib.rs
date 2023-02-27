#![cfg_attr(test, no_main)]
#![no_std]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(abi_x86_interrupt)]

use core::panic::PanicInfo;
pub use core::format_args;

#[macro_use]
pub mod vga;

mod interrupts;
mod gdt;

pub mod qemu;

#[macro_use]
mod serial;

pub fn init() {
    unsafe {
        gdt::init();
        interrupts::init();
    }
}

pub fn test_runner(tests: &[&dyn Test]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }
}

pub trait Test {
    fn run(&self);
}

impl<T> Test for T 
    where T: Fn()
{
    fn run(&self) {
        serial_print!("{}... ", core::any::type_name::<Self>().split("::").last().unwrap_or(core::any::type_name::<Self>()));
        self();
        serial_println!("[Ok]");
    }
}

pub fn spin() -> ! {
    loop {
        core::hint::spin_loop();
        x86_64::instructions::hlt();
    }
}

#[no_mangle]
#[cfg(test)]
extern "C" fn _start() -> ! {
    init();
    
    test_main();

    qemu::exit_qemu(qemu::QemuExitCode::Success);
}

pub fn test_panic(info: &PanicInfo) -> ! {
    serial_println!("[FAILED]");
    serial_println!("{}", info);

    qemu::exit_qemu(qemu::QemuExitCode::Failed);
}