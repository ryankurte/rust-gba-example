#![feature(lang_items)]
#![no_std]

extern crate gba as gba;
use gba::gfx::{Mode3, Colour};

use core::ptr;
use core::slice;

// [0x48, 0x65, 0x6c, 0x6c, 0x6f, 0x2c, 0x20, 0x77, 0x6f, 0x72, 0x6c, 0x64, 0x21]
static RODATA: &str = "Hello, world!";

static mut BSS: u32 = 0;
static mut DATA: u32 = 1;

fn main() {
    let mut g = Mode3::new();

    g.enable();

    for x in 0..240 {
        for y in 45..55 {
            g.set(x, y, Colour::rgb(0xFF, 0x00, 0x00));
        }

        for y in 65..75 {
            g.set(x, y, Colour::rgb(0x00, 0xFF, 0x00));
        }

        for y in 85..95 {
            g.set(x, y, Colour::rgb(0x00, 0x00, 0xFF));
        }
    }

    // Don't forget the forever loop!!
    loop {};
}

