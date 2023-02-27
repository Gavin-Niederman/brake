#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(brake_kernel::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

use brake_kernel::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();

    brake_kernel::qemu::exit_qemu(brake_kernel::qemu::QemuExitCode::Success);
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    brake_kernel::test_panic(info)
}

#[test_case]
fn test_println() {
    println!("test_println output");
}