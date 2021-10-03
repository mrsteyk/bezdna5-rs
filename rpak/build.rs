extern crate cc;

use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=decomp.hh");
    println!("cargo:rerun-if-changed=decomp.cc");

    cc::Build::new()
        .cpp(true)
        .file("src/decomp.cc")
        .compile("decomp");
}
