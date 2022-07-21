#![allow(unused_variables)]
use std::env;

fn main() {
    let libdir = env::var("MAA_LIB_PATH").unwrap();

    println!("cargo:rustc-flags=-L {}", libdir);
    println!("cargo:rustc-link-search=native={}", libdir);
    let out_dir = env::var("OUT_DIR").unwrap();
}
