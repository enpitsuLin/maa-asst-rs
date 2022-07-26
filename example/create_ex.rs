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
    let msg = AsstMsg::try_from(msg).unwrap();
    let detail_json = CStr::from_ptr(detail_json).to_str().unwrap();
    println!("回调消息: {:?} {:?} {:?}", msg, detail_json, custom_arg);
}

fn main() {
    // 读取构建用的环境变量
    let resource_path = env!("MAA_LIB_PATH");

    print!("{}", resource_path);
    let loaded = load_resource(resource_path).unwrap();
    if !loaded {
        println!("load resource failed");
        pause();
        return;
    }

    let custom_arg: *mut c_void = &mut String::from("") as *mut _ as *mut c_void;
    let ptr = create_ex(Some(callback), custom_arg);

    let connected = connect(ptr, "adb", "127.0.0.1:59216", None).unwrap();

    if !connected {
        println!("connect failed");
        destroy(&ptr);
        pause();
        return;
    }

    append_task(ptr, "StartUp", "").unwrap();
    append_task(ptr, "Fight", r#"{"stage": "1-7","times":1}"#).unwrap();
    append_task(ptr, "Visit", "").unwrap();

    pause();
    start(ptr);
    stop(ptr);
    destroy(&ptr);
}
