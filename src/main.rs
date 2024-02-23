#![no_std] // Don't link the Rust Standard Library
#![no_main] // Disable all Rust-level entry points
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]


use core::panic::PanicInfo;
mod vga_buffer;
mod serial;

static HELLO: &[u8] = b"Hello World!";

// Disable name mangling to ensure that the Rust compiler really outputs a function with the name _start
// w/o this attribute, the function would generate a name like _ZN4core3ptr18
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");


    #[cfg(test)]
    test_main();

    loop {}
}

/// This function is called on panic when not in test mode
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    loop {}
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}


#[cfg(test)]
pub fn test_runner(tests: &[&dyn Fn()]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }

    exit_qemu(QemuExitCode::Success);
}

#[test_case]
fn trivial_assertion() {
    serial_print!("trivial assertion... ");
    assert_eq!(0, 1);
    serial_println!("[ok]");
}