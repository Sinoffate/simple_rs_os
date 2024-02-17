#![no_std] // Don't link the Rust Standard Library
#![no_main] // Disable all Rust-level entry points

use core::panic::PanicInfo;

// Disable name mangling to ensure that the Rust compiler really outputs a function with teh name _start
// w/o this attribute, the function would generate a name like _ZN4core3ptr18
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // this function is the entry point, since the linker looks for a function
    // named `_start` by default
    loop {}
}

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}


