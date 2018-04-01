#![feature(lang_items)]
#![no_std]

extern crate gba as gba;
use gba::gfx::{Mode3, Colour};

fn main() {
    let g = Mode3::new();

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

