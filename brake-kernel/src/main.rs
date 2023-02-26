#![no_main]
#![no_std]

use brake_kernel::{spin, init};

use brake_kernel::{print, println};

#[no_mangle]
extern "C" fn _start() -> ! {
    init();
    
    print!("No newline:");
    println!("Hello, world!");

    // Reference?
    fn stack_overflow() {
        stack_overflow()
    }

    stack_overflow();

    spin();
}
