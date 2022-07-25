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
    let dependencies = vec!["/*.dll"];
    for item in dependencies.iter() {
        let mut from = path.clone();
        from.push_str(item);
        for entry in glob::glob(&from).expect("路径匹配有误") {
            match entry {
                Ok(from_file) => {
                    if from_file.is_dir() {
                        panic!("请不要将文件夹命名成*.dll形式")
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
