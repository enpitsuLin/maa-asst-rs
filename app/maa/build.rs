#![cfg_attr(any(not(target_os = "macos")), allow(unused))]

extern crate embed_resource;

use std::env;

fn main() {
    let target = env::var("CARGO_CFG_TARGET_OS");

    println!("cargo::rustc-check-cfg=cfg(gles)");

    match target.as_deref() {
        Ok("macos") => {
            #[cfg(target_os = "macos")]
            println!("cargo:warning=macos bundle todo")
        },
        #[cfg(target_os = "windows")]
        Ok("windows") => {
            let rc_file = std::path::Path::new("res/windows/maa-manifest.rc");
            println!("cargo:rerun-if-changed={}", rc_file.display());
            embed_resource::compile(rc_file, embed_resource::NONE)
                .manifest_required()
                .unwrap();
        },
        _ => ()
    };
}
