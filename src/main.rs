#![feature(lang_items)]
#![no_std]

extern crate gba as gba;
use gba::graphics::Graphics;
use gba::graphics::bitmap::{BitmapMode, Mode3, Mode5};
use gba::graphics::colour::Colour;
use gba::graphics::helpers::rainbow;

const max: u8 = 0x1F;

fn main() {
    let mut m5 = Graphics::<Mode5>::new();

    {
        let mut g = m5.active();

        rainbow(g, 10);
        
        g.swap();
    }

    let _ = m5.mode3();

    // Don't forget the forever loop!!
    loop {};
}

