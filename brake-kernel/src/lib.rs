#![no_std]
#![feature(abi_x86_interrupt)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]

#[macro_use]
pub mod vga;

mod interrupts;
pub mod gdt;

use core::panic::PanicInfo;
pub use core::format_args;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{info}");
    spin()
}

pub fn spin() -> ! {
    loop {
        core::hint::spin_loop();
        x86_64::instructions::hlt();
    }
}

#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running tests");
    for test in tests {
        test();
    }
}

pub fn init() {
    unsafe {
        gdt::init();
        interrupts::init();
    }
}