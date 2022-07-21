#![allow(unused_variables)]
use std::env;

fn main() {
    let libdir = env::var("MAA_LIB_PATH").unwrap_or_else(|err| {
        println!("环境变量 MAA_LIB_PATH 未设置");
        panic!("{:?}", err);
    });

    println!("cargo:rustc-flags=-L {}", libdir);
    println!("cargo:rustc-link-search=native={}", libdir);
}
