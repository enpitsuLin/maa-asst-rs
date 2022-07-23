#![allow(unused_variables, unused_imports)]
extern crate glob;

use std::{
    env, fs,
    path::{Path, PathBuf},
};

fn find_library_and_copy(path: String) -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = env::var("OUT_DIR")?;
    let dependencies = vec!["/*.dll", "/resource/"];
    for item in dependencies.iter() {
        let mut from = path.clone();
        from.push_str(item);
        for entry in glob::glob(&from).expect("路径匹配有误") {
            match entry {
                Ok(from_file) => {
                    if from_file.is_dir() {
                        copy(from_file, out_dir.to_owned() + "/resource")?;
                    } else {
                        let out_filename = from_file.file_name().unwrap().to_str().unwrap();
                        let to_file = out_dir.to_owned() + "/" + out_filename;
                        fs::copy(from_file, to_file.clone()).unwrap_or_else(|err| {
                            panic!("{:?} {:?}", err, to_file);
                        });
                    };
                }
                Err(_) => {
                    todo!()
                }
            }
        }
    }
    Ok(())
}

pub fn copy<U: AsRef<Path>, V: AsRef<Path>>(from: U, to: V) -> Result<(), std::io::Error> {
    let mut stack = Vec::new();
    stack.push(PathBuf::from(from.as_ref()));

    let output_root = PathBuf::from(to.as_ref());
    let input_root = PathBuf::from(from.as_ref()).components().count();

    while let Some(working_path) = stack.pop() {
        println!("process: {:?}", &working_path);

        // Generate a relative path
        let src: PathBuf = working_path.components().skip(input_root).collect();

        // Create a destination if missing
        let dest = if src.components().count() == 0 {
            output_root.clone()
        } else {
            output_root.join(&src)
        };
        if fs::metadata(&dest).is_err() {
            println!(" mkdir: {:?}", dest);
            fs::create_dir_all(&dest)?;
        }

        for entry in fs::read_dir(working_path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                stack.push(path);
            } else {
                match path.file_name() {
                    Some(filename) => {
                        let dest_path = dest.join(filename);
                        println!("  copy: {:?} -> {:?}", &path, &dest_path);
                        fs::copy(&path, &dest_path)?;
                    }
                    None => {
                        println!("failed: {:?}", path);
                    }
                }
            }
        }
    }

    Ok(())
}

fn main() {
    let libdir = env::var("MAA_LIB_PATH").unwrap_or_else(|err| {
        println!("环境变量 MAA_LIB_PATH 未设置");
        panic!("{:?}", err);
    });

    println!("cargo:rustc-flags=-L {}", libdir);
    println!("cargo:rustc-link-search=native={}", libdir);
    println!("cargo:rerun-if-changed=build.rs");

    if cfg!(feature = "pack") {
        find_library_and_copy(libdir).unwrap();
    }
}
