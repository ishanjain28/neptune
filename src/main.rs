#![no_std] // don't include rust std lib
#![no_main] // disable all rust entry point

#[macro_use]
extern crate lazy_static;

mod vga_buffer;

use core::panic::PanicInfo;
use vga_buffer::{Buffer, Color, ColorCode, Writer};

/// This function is called on panic.
#[panic_handler]
pub fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[no_mangle] // don't change the name of function in symbol table
pub extern "C" fn _start() -> ! {
    use core::fmt::Write;
    vga_buffer::WRITER.lock().write_str("Hello again").unwrap();
    println!(", some numbers: {} {}", 42, 1.337);

    panic!("panic message goes right here");

    loop {}
}
