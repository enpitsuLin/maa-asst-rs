use libc::{c_char, c_int, c_void};
use maa::*;
use std::ffi::CStr;
use std::io::prelude::*;
use std::{env, io};

/// 参考 https://users.rust-lang.org/t/rusts-equivalent-of-cs-system-pause/4494/4
fn pause() {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    // We want the cursor to stay at the end of the line, so we print without a newline and flush manually.
    write!(stdout, "Press any key to continue...").unwrap();
    stdout.flush().unwrap();

    // Read a single byte and discard
    let _ = stdin.read(&mut [0u8]).unwrap();
}

/// 消息回调函数 简单输出下
unsafe extern "C" fn callback(msg: c_int, detail_json: *const c_char, custom_arg: *mut c_void) {
    println!(
        "rust callback: {:?} {:?}",
        CStr::from_ptr(detail_json).to_str(),
        detail_json,
    );
}

fn main() {
    // 默认读取构建的可执行文件同目录中的resource文件夹
    let cur_dir = env::current_dir().unwrap();
    let resource_path = cur_dir.to_str().unwrap();

    let loaded = load_resource(resource_path).unwrap();
    if !loaded {
        println!("load resource failed");
        pause();
        return;
    }

    let custom_arg: *mut c_void = &mut String::from("") as *mut _ as *mut c_void;
    let ptr = create_ex(Some(callback), custom_arg);

    let connected = connect(ptr, "adb", "127.0.0.1:52729", None).unwrap();

    if !connected {
        println!("connect failed");
        destroy(ptr);
        pause();
        return;
    }

    append_task(ptr, "StartUp", "").unwrap();
    append_task(ptr, "Fight", r#"{"stage": "1-7","times":1}"#).unwrap();
    append_task(ptr, "Visit", "").unwrap();
    start(ptr);
    pause();
    stop(ptr);
    destroy(ptr);
}
