#![no_std] // Don't link the Rust Standard Library
#![no_main] // Disable all Rust-level entry points

use core::panic::PanicInfo;
mod vga_buffer;

static HELLO: &[u8] = b"Hello World!";

// Disable name mangling to ensure that the Rust compiler really outputs a function with the name _start
// w/o this attribute, the function would generate a name like _ZN4core3ptr18
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");
    panic!("Some panic message");

    loop {}
}

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

