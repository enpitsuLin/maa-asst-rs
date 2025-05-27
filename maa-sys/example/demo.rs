use maa_sys::task::{FightTask, StartUpTask};
use maa_sys::Assistant;
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
    let resource_path = env!("MAA_RESOUCE_PATH");
    let mut assistant = Assistant::new(resource_path).unwrap();

    assistant.connect("adb", "127.0.0.1:62001", None);

    if !assistant.is_connected() {
        println!("connect failed");
        drop(assistant);
        pause();
        return;
    }

    assistant.append_task(
        StartUpTask::builder()
            .enable(true)
            .client_type("Official")
            .start_game_enabled(true)
            .account_name("123****4567")
            .build(),
    );
    assistant.append_task(
        FightTask::builder()
            .enable(true)
            .stage("1-7")
            .medicine(3)
            .times(5)
            .build(),
    );
    assistant.start();
    pause();
    assistant.stop();
    drop(assistant);
}
