#![feature(lang_items)]
#![no_std]

extern crate gba as gba;
use gba::graphics::Graphics;
use gba::graphics::bitmap::{BitmapMode, Mode3};
use gba::graphics::colour::Colour;

const max: u8 = 0x1F;

fn main() {
    let mut gx = Graphics::new();

    let rainbow: [Colour; 6] = [
        Colour::R,
        Colour::O,
        Colour::Y,
        Colour::G,
        Colour::B,
        Colour::V,
    ];

    {
        let mut g = gx.active();

        for x in 0..240 {

            let start = 160 / 2 - rainbow.len() * 10 / 2;

            for c in 0..rainbow.len() {
                let offset = start + 10 * c;
                for y in offset..offset+10 {
                    g.set(x, y, rainbow[c].u16());
                }
            }
        }
    }

    gx = gx.mode3();

    // Don't forget the forever loop!!
    loop {};
}

