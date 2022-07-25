#![allow(unused_variables)]
use std::{
    env, fs,
    path::{Path, PathBuf},
};

fn main() {
    let libdir = env::var("MAA_LIB_PATH").unwrap_or_else(|err| {
        println!("cargo:warning=环境变量 MAA_LIB_PATH 未设置");
        panic!("{:?}", err);
    });

    println!("cargo:rustc-flags=-L {}", libdir);
    println!("cargo:rustc-link-search=native={}", libdir);

    println!("cargo:warning=移动依赖");
    let output_path = get_output_path();
    find_library_and_copy(libdir, output_path).unwrap();
}

fn get_output_path() -> PathBuf {
    //<root or manifest path>/target/<profile>/
    let manifest_dir_string = env::var("CARGO_MANIFEST_DIR").unwrap();
    let build_type = env::var("PROFILE").unwrap();
    let path = Path::new(&manifest_dir_string)
        .join("target")
        .join(build_type);
    return PathBuf::from(path);
}

fn find_library_and_copy(path: String, out_dir: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:warning={:?}", out_dir);
    let dependencies = vec!["/*.dll", "/resource/"];
    for item in dependencies.iter() {
        let mut from = path.clone();
        from.push_str(item);
        for entry in glob::glob(&from).expect("路径匹配有误") {
            match entry {
                Ok(from_file) => {
                    if from_file.is_dir() {
                        let mut out_dir = out_dir.clone();
                        out_dir.push("./resource");
                        copy(from_file, out_dir)?;
                    } else {
                        let out_filename = from_file.file_name().unwrap().to_str().unwrap();
                        let mut out_dir = out_dir.clone();
                        out_dir.push(format!("./{}", out_filename).as_str());
                        let to_file = out_dir;
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
