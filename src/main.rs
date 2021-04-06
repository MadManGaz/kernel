#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(kernel::test_runner)]
#![reexport_test_harness_main = "test_main"]

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use kernel::println;

// This allows or type checking the method signature of the entry point so we
// don't accidentally pass in arbitrary arguments. `_start` is defined lower
// in the programs hierarchy.
entry_point!(kernel_main);

/// Entry-point for the kernel.
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use kernel::memory;
    use x86_64::{
        structures::paging::{Page, Translate},
        VirtAddr,
    };

    println!("Hello Friends{}", "!");
    kernel::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = memory::EmptyFrameAllocator;

    // map an unused page
    let page = Page::containing_address(VirtAddr::new(0));
    memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);

    // write the string `New!` to the screen through the new mapping
    let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    unsafe { page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e) };

    #[cfg(test)]
    test_main();

    println!("We did not crash! :^)");
    kernel::hlt_loop();
}

/// Custom panic handler writes to a VGA buffer rather than stdout. This panic
/// handler is used when running normally through `cargo run`, or on a real
/// machine.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    kernel::hlt_loop();
}

/// Panic handler for running tests in a QEMU virtual machine. Error messages
/// are instead sent through a serial device to be displayed on the host when
/// using `cargo test`.
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    kernel::test_panic_handler(info)
}
