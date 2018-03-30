#![feature(lang_items)]
#![no_std]

extern crate gba_rs as gba;

use core::ptr;
use core::slice;

// [0x48, 0x65, 0x6c, 0x6c, 0x6f, 0x2c, 0x20, 0x77, 0x6f, 0x72, 0x6c, 0x64, 0x21]
static RODATA: &str = "Hello, world!";

static mut BSS: u32 = 0;
static mut DATA: u32 = 1;

fn main() {
    // force the compiler the keep these symbols in the final program
    unsafe {      
        *(0x04000000 as *mut u32)= 0x0403;
        let buff : &mut [u16] = slice::from_raw_parts_mut(0x06000000 as *mut u16, 240 * 160);

        buff[120+80*240] = 0x001F;
        buff[136+80*240] = 0x03E0;
        buff[120+96*240] = 0x7C00;
    }
}
