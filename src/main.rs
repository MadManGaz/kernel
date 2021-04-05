#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(kernel::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use kernel::println;

/// Entry-point for the kernel.
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    kernel::init();

    #[cfg(test)]
        test_main();

    loop {}
}

/// Custom panic handler writes to a VGA buffer rather than stdout. This panic
/// handler is used when running normally through `cargo run`, or on a real
/// machine.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

/// Panic handler for running tests in a QEMU virtual machine. Error messages
/// are instead sent through a serial device to be displayed on the host when
/// using `cargo test`.
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    kernel::test_panic_handler(info)
}
