#![allow(unused_variables)]
use std::env;

fn main() {
    let libdir = "C:/Users/user/Desktop/MaaBundle-Dev-2022-06-27-08-15-29-de43dd3/";
    
    println!("cargo:rustc-flags=-L {}", libdir);
    println!("cargo:rustc-link-search=native={}", libdir);
    let out_dir = env::var("OUT_DIR").unwrap();
}
