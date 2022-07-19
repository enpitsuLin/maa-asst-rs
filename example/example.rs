use maa::*;
use std::io;
use std::io::prelude::*;

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
    let loaded = load_resource("path").unwrap();
    if !loaded {
        println!("load resource failed");
        return;
    }
    let ptr = create();

    let connected = connect(ptr, "adb", "127.0.0.1", "").unwrap();

    if !connected {
        println!("connect failed");
        destroy(ptr);
        return;
    }

    append_task(ptr, "StartUp", "").unwrap();
    append_task(ptr, "Fight", r#"{"stage": "1-7"}"#).unwrap();
    start(ptr);
    pause();
    stop(ptr);
    destroy(ptr);
}
