use maa::*;
use std::io::prelude::*;
use std::path::Path;
use std::{env, io};

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
    // 默认读取构建的可执行文件同目录中的resource文件夹
    let cur_dir = env::current_dir().unwrap();
    let resource_path = cur_dir.to_str().unwrap();

    let loaded = load_resource(resource_path).unwrap();
    if !loaded {
        println!("load resource failed");
        pause();
        return;
    }
    let ptr = create();

    let connected = connect(ptr, "adb", "127.0.0.1", "").unwrap();

    if !connected {
        println!("connect failed");
        destroy(ptr);
        pause();
        return;
    }

    append_task(ptr, "StartUp", "").unwrap();
    append_task(ptr, "Fight", r#"{"stage": "1-7"}"#).unwrap();
    start(ptr);
    pause();
    stop(ptr);
    destroy(ptr);
}
