#![no_std] // don't include rust std lib
#![no_main] // disable all rust entry point

extern crate spin;
extern crate volatile;

#[macro_use]
extern crate lazy_static;

mod vga_buffer;

use core::fmt::Write;
use core::panic::PanicInfo;
use vga_buffer::{Buffer, Color, ColorCode, Writer};

/// This function is called on panic.
#[panic_handler]
#[no_mangle]
pub fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle] // don't change the name of function in symbol table
pub extern "C" fn _start() -> ! {
    use core::fmt::Write;
    vga_buffer::WRITER.lock().write_str("Hello again").unwrap();
    write!(
        vga_buffer::WRITER.lock(),
        ", some numbers: {} {}",
        42,
        1.337
    )
    .unwrap();

    loop {}
}
