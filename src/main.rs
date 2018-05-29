#![feature(lang_items)]
#![no_std]

extern crate gba as gba;
use gba::input::{IO, Keys};
use gba::graphics::Graphics;
use gba::graphics::bitmap::{BitmapMode, Mode3, Mode4, Mode5};
use gba::graphics::colour::Colour;
use gba::graphics::helpers::{rainbow, rainbow2};

const max: u8 = 0x1F;

fn main() {
    let mut io = IO::new();
    let mut m = Graphics::<Mode4>::new();
    let mut g = m.active();

    loop {
        io.update();

        let mut width = 10;
        if io.is_pressed(Keys::Up) && width < 40 {
            width += 1;
        }
        if io.is_pressed(Keys::Down) && width > 2 {
            width -= 1;
        }

        g.clear();
        rainbow2(g, width);

        while !g.vsync() {};
        g.swap();
    }

}

