//! # Build script
//!

use std::{env, path::PathBuf};
extern crate bindgen;

fn main() {
    println!("cargo:rerun-if-env-changed=MAA_HEADER_PATH");

    let out_dir = env::var("OUT_DIR").unwrap();

    let maa_header_path = env::var("MAA_HEADER_PATH").unwrap();

    let bindings = bindgen::Builder::default()
        .header(maa_header_path)
        .dynamic_library_name("MaaCore")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(out_dir);
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
