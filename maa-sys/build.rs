//! # Build script
//!

use std::{
    env,
    path::{Path, PathBuf},
};
extern crate bindgen;
extern crate glob;

fn main() {
    println!("cargo:rerun-if-env-changed=MAA_LIB_PATH");
    println!("cargo:rerun-if-env-changed=MAA_HEADER_PATH");

    let lib_dir = env::var("MAA_LIB_PATH").unwrap_or_else(|err| {
        println!("cargo:warning=环境变量 MAA_LIB_PATH 未设置");
        panic!("{:?}", err);
    });
    
    let out_dir = env::var("OUT_DIR").unwrap();

    println!("cargo:rustc-link-search=native={}", lib_dir);

    println!("cargo:rustc-link-lib=static=MaaCore");

    let maa_header_path = env::var("MAA_HEADER_PATH").unwrap();

    let bindings = bindgen::Builder::default()
        .header(maa_header_path)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    println!("cargo:warning=移动依赖");
    let work_dir = vec!["../../..", "../../../deps"];

    work_dir.iter().for_each(|dir_path| {
        let asset_path = Path::new(&lib_dir).join("*.{dll,dylib}");
        let exe_dir = &Path::new(&out_dir[..]).join(dir_path).canonicalize().unwrap();
        symbolic_link_assets(asset_path.to_str().unwrap(), exe_dir).unwrap();
    });
}

#[cfg(windows)]
fn symbolic_link_assets(asset_dir: &str, exe_dir: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    use std::{fs, os};

    for asset in glob::glob(asset_dir).unwrap() {
        match asset {
            Ok(dll_origin) => {
                if dll_origin.is_dir() {
                    panic!("请不要将文件夹命名成*.dll形式 {:?}", dll_origin)
                } else {
                    let dll_filename = dll_origin.file_name().unwrap().to_str().unwrap();
                    let dll_symbol = exe_dir.join(dll_filename);
                    if dll_symbol.exists() {
                        fs::remove_file(dll_symbol.clone())?;
                    }
                    os::windows::fs::symlink_file(dll_origin, dll_symbol)?;
                };
            },
            Err(_) => unreachable!(),
        }
    }
    Ok(())
}

#[cfg(target_os = "macos")]
fn symbolic_link_assets(asset_dir: &str, exe_dir: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    use std::{fs, os::unix::fs::symlink};

    for asset in glob::glob(asset_dir).unwrap() {
        match asset {
            Ok(dylib_origin) => {
                if dylib_origin.is_dir() {
                    panic!("请不要将文件夹命名成*.dylib形式 {:?}", dylib_origin)
                } else {
                    let dylib_filename = dylib_origin.file_name().unwrap().to_str().unwrap();
                    let dylib_symbol = exe_dir.join(dylib_filename);
                    if dylib_symbol.exists() {
                        fs::remove_file(dylib_symbol.clone())?;
                    }
                    symlink(dylib_origin, dylib_symbol)?;
                };
            },
            Err(_) => unreachable!(),
        }
    }
    Ok(())
}

#[cfg(not(any(windows, target_os = "macos")))]
fn symbolic_link_assets(asset_dir: &str, exe_dir: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    unimplemented!()
}
