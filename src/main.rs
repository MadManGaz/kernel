#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(kernel::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use alloc::{boxed::Box, rc::Rc, vec, vec::Vec};
use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use kernel::println;
use x86_64::VirtAddr;

// This allows or type checking the method signature of the entry point so we
// don't accidentally pass in arbitrary arguments. `_start` is defined lower
// in the programs hierarchy.
entry_point!(kernel_main);

/// Entry-point for the kernel.
fn kernel_main(_boot_info: &'static BootInfo) -> ! {
    use kernel::allocator;
    use kernel::memory::{self, BootInfoMemoryAllocator};
    println!("Hello Friends{}", "!");
    kernel::init();

    let phys_mem_offset = VirtAddr::new(_boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoMemoryAllocator::init(&_boot_info.memory_map) };

    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");

    // allocate a number on the heap.
    let heap_value = Box::new(41);
    println!("heap_value at {:p}", heap_value);

    // create a dynamically sized vector
    let mut vec = Vec::new();
    for i in 0..500 {
        vec.push(i * 2);
    }
    println!("vec at {:p}", vec.as_slice());

    for (index, number) in vec.iter().enumerate() {
        println!("vec[{}] = {}", index, number);
    }

    // create a reference counted vector -> will be freed when count reaches 0
    let reference_counted = Rc::new(vec![1, 2, 3]);
    let cloned_reference = reference_counted.clone();
    println!(
        "current reference count is {}",
        Rc::strong_count(&cloned_reference)
    );
    core::mem::drop(reference_counted);
    println!(
        "reference count is {} now",
        Rc::strong_count(&cloned_reference)
    );

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
