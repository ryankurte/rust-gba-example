#![feature(lang_items)]
#![no_std]

extern crate gba as gba;
use gba::gfx::{Mode3, Colour};

const max: u8 = 0x1F;

fn main() {
    let g = Mode3::new();

    g.enable();

    let rainbow: [Colour; 6] = [
        Colour::rgb(max, 0, 0),
        Colour::rgb(max, max/2, 0),
        Colour::rgb(max, max, 0),
        Colour::rgb(0, max, 0),
        Colour::rgb(0, 0, max),
        Colour::rgb(max/2, 0, max),
    ];


    for x in 0..240 {

        let start = 160 / 2 - rainbow.len() * 10 / 2;

        for c in 0..rainbow.len() {
            let offset = start + 10 * c;
            for y in offset..offset+10 {
                g.set(x, y, rainbow[c].clone());
            }
        }
    }

    // Don't forget the forever loop!!
    loop {};
}

