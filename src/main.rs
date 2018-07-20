#![feature(panic_implementation)] // required to define panic handler
#![no_std] // don't include rust std lib
#![no_main] // disable all rust entry point

use core::panic::PanicInfo;

/// This function is called on panic.
#[panic_implementation]
#[no_mangle]
pub fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle] // don't change the name of function in symbol table
pub extern "C" fn _start() -> ! {
    loop {}
}
