#![no_std] // Freestanding binaries can't have a stdlib
#![no_main] // Rust main requires a runtime. We can't have that.

use core::panic::PanicInfo;

static HELLO: &[u8] = b"Hello World!";

//We don't want to mangle the name as this will obfuscate what
// function to call for the multiboot stage.
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // This is the entry point to the program. Will be called from
    // the multiboot assembly. 
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    loop { }
}

// Rust requires a panic handler to be defined. This usually
// comes from stdlib, but we don't have that here.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop { }
}
