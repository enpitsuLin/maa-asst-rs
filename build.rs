#![allow(unused_variables)]
use std::{
    env, fs, os,
    path::{Path, PathBuf},
};

fn main() {
    let lib_dir = env::var("MAA_LIB_PATH").unwrap_or_else(|err| {
        println!("cargo:warning=环境变量 MAA_LIB_PATH 未设置");
        panic!("{:?}", err);
    });

    let out_dir = env::var("OUT_DIR").unwrap();

    println!("cargo:rustc-flags=-L {}", lib_dir);
    println!("cargo:rustc-link-search=native={}", lib_dir);

    println!("cargo:warning=移动依赖");
    let work_dir = vec!["../../..", "../../../deps"];

    work_dir.iter().for_each(|dir_path| {
        let asset_path = Path::new(&lib_dir).join("*.dll");
        let exe_dir = &Path::new(&out_dir[..])
            .join(dir_path)
            .canonicalize()
            .unwrap();
        symbolic_link_assets(asset_path.to_str().unwrap(), exe_dir).unwrap();
    });
}

#[cfg(windows)]
fn symbolic_link_assets(
    asset_dir: &str,
    exe_dir: &PathBuf,
) -> Result<(), Box<dyn std::error::Error>> {
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
            }
            Err(_) => unreachable!(),
        }
    }
    Ok(())
}

#[cfg(not(windows))]
fn symbolic_link_assets(
    asset_dir: &str,
    exe_dir: &PathBuf,
) -> Result<(), Box<dyn std::error::Error>> {
    unimplemented!()
}
