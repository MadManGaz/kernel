// Freestanding binaries can't have a stdlib
#![no_std]
// Rust main requires a runtime. We can't have that.
#![no_main]
// The built in testing framework does not work without stdlib. We have
// to roll our own.
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
// We have to rename the generated test runner function from main() so we
// can call it from our custom entry point.
#![reexport_test_harness_main = "test_main"]

mod serial;
mod vga_buffer;

use core::panic::PanicInfo;

/// Entry-point.
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}\nVery Cool!", "!");

    #[cfg(test)]
    test_main();

    loop {}
}

pub trait Testable {
    fn run(&self) -> ();
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        serial_print!("{}...\t", core::any::type_name::<T>());
        self();
        serial_println!("[ok]");
    }
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
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

// This is the definition of a custom testing framework for kernel
// development.
#[cfg(test)]
fn test_runner(tests: &[&dyn Testable]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }
    exit_qemu(QemuExitCode::Success);
}

/// The status code we send to our host.
/// This is handled by the bootloader crate to return a 0 or 1 status code
/// depending on if tests pass or not.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

/// Should only be used when running tests. Otherwise we are making assumptions
/// on what sort of machine we are running on.
pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}
