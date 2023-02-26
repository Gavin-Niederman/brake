#![no_main]
#![no_std]

use brake_kernel::{spin, init};

use brake_kernel::println;

#[no_mangle]
extern "C" fn _start() -> ! {
    init();
    
    println!("Hello, world!");

    spin();
}
