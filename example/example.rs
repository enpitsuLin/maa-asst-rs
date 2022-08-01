use maa::*;
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

fn main() {
    // 读取构建用的环境变量
    let resource_path = env!("MAA_LIB_PATH");

    let loaded = load_resource(resource_path).unwrap();
    if !loaded {
        println!("load resource failed");
        pause();
        return;
    }
    let mut ptr = create();

    let connected = connect(ptr, "adb", "127.0.0.1", None).unwrap();

    if !connected {
        println!("connect failed");
        destroy(&mut ptr);
        pause();
        return;
    }

    append_task(ptr, "StartUp", "").unwrap();
    append_task(ptr, "Fight", r#"{"stage": "1-7"}"#).unwrap();
    start(ptr);
    pause();
    stop(ptr);
    destroy(&mut ptr);
}
