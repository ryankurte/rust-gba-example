use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;

extern crate gcc;

fn main() {
    // Put the linker scripts somewhere the linker can find it
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());

    // Copy linker script into output dir
    File::create(out.join("gba_cart.ld"))
        .unwrap()
        .write_all(include_bytes!("src/gba_cart.ld"))
        .unwrap();
    File::create(out.join("gba.specs"))
        .unwrap()
        .write_all(include_bytes!("src/gba.specs"))
        .unwrap();

    // Build the crt0 / startup script
    let mut build = gcc::Build::new();
    build.compiler("arm-none-eabi-gcc")
          .archiver("arm-none-eabi-ar");
    build.flag("-mcpu=arm7tdmi");
    build.flag("-mthumb-interwork");
    build.flag("-nostdlib");

    build.clone().file("src/gba_crt0.s").compile("gba_crt0");
    //build.clone().file("src/gba_main.c").compile("gba_main.o");

    // This doesn't seem to pick up the right targets...
    //Command::new("gcc").args(&["src/gbafix.c", "-o"])
    //                   .arg(&format!("{}/gbafix", out.to_str().unwrap()))
    //                   .status().unwrap();
                       
    println!("cargo:rustc-link-search={}", out.display());
    println!("cargo:rerun-if-changed=src/gba_cart.ld");
    println!("cargo:rerun-if-changed=src/gba_crt0.s");
}
