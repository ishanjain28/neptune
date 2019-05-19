// don't include rust std lib
#![no_std]
// disable all rust entry point
#![no_main]
// Utilise custom test frameworks feature in rust, Since the default one doesn't works in no_std environments
#![feature(custom_test_frameworks)]
// Specify the runner, aka, The function that will execute all the tests
#![test_runner(neptune::test_runner)]
// custom test frameworks creates an entry point called `main`. Since we are using no_main and have
// set a custom entry point, The entry point created by custom test framework gets ignored.
// This changes the name of the entry point to test_main which can then be called in _start
#![reexport_test_harness_main = "test_main"]

#[macro_use]
extern crate lazy_static;

mod serial;
mod vga_buffer;

use core::panic::PanicInfo;

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
pub fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    neptune::test_panic_handler(info)
}

#[no_mangle] // don't change the name of function in symbol table
pub extern "C" fn _start() -> ! {
    println!("Hello! Here are some numbers: {} {}", 42, 1.337);

    #[cfg(test)]
    test_main();

    loop {}
}

#[test_case]
fn trivial_assertion() {
    print!("trivial assertion... ");
    serial_print!("trivial assertion... ");
    assert_eq!(1, 1);
    println!("[ok]");
    serial_println!("[ok]");
}
