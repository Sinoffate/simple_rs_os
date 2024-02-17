#![no_std] // Don't link the Rust Standard Library
#![no_main] // Disable all Rust-level entry points

use core::panic::PanicInfo;

static HELLO: &[u8] = b"Hello World!";

// Disable name mangling to ensure that the Rust compiler really outputs a function with teh name _start
// w/o this attribute, the function would generate a name like _ZN4core3ptr18
#[no_mangle]
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    loop {}
}

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}


