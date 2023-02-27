#![no_main]
#![no_std]
#![feature(custom_test_frameworks)]
#![test_runner(brake_kernel::test_runner)]

use brake_kernel::spin;
use brake_kernel::println;
use core::panic::PanicInfo;

#[no_mangle]
extern "C" fn _start() -> ! {
    brake_kernel::init();
    
    println!("Hello, world!");

    spin();
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    brake_kernel::test_panic(info)
}

#[test_case]
fn overflow_vga_buffer() {
    for i in 1..=100 {
        println!("Line {} out of 100", i);
    }
}

#[test_case]
fn should_fail() {
    panic!();
}