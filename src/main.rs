#![feature(lang_items)]
#![feature(asm)]
#![no_std]

extern crate gba as gba;
use gba::input::{IO, Keys};
use gba::graphics::Graphics;
use gba::graphics::bitmap::{BitmapMode, Mode4};
use gba::graphics::helpers::{rainbow2};

const max: u8 = 0x1F;

fn wait(n: usize) {
    for _i in 0 .. n {
        unsafe {
            asm!("nop");
        }
    }
}

fn main() {
    let mut io = IO::new();
    let mut m = Graphics::<Mode4>::new();
    let mut g = m.active();

    let mut width = 10;

    loop {
        io.update();
        if io.is_pressed(Keys::Up) && width < 20 {
            width += 1;
        }
        if io.is_pressed(Keys::Down) && width > 2 {
            width -= 1;
        }
    
        g.clear();
        rainbow2(g, width);

        while g.vblank() {};
        g.swap();
    }

}

