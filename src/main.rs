#![feature(lang_items)]
#![no_std]

extern crate gba_rs as gba;

use core::ptr;

// [0x48, 0x65, 0x6c, 0x6c, 0x6f, 0x2c, 0x20, 0x77, 0x6f, 0x72, 0x6c, 0x64, 0x21]
static RODATA: &str = "Hello, world!";

static mut BSS: u32 = 0;
static mut DATA: u32 = 1;

fn main() {
    // force the compiler the keep these symbols in the final program
    unsafe {
        ptr::read_volatile(RODATA.as_bytes().as_ptr());
        ptr::read_volatile(&BSS);
        ptr::read_volatile(&DATA);
    }
}
