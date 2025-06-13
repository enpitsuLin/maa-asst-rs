use maa_sys::task::{FightTask, StartUpTask};
use maa_sys::{Assistant, Connection, InstanceOptionKey};
use std::env;

fn pause() {
    println!("按任意键继续...");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resource_path = env!("MAA_RESOURCE_PATH");
    let mut assistant = Assistant::new(resource_path)?;

    // 未 Root 的设备使用 adb 模式
    assistant.set_instance_option(InstanceOptionKey::TouchMode, "adb")?;
    assistant.connect(Connection::adb("adb", "192.168.20.29:40351"), None)?;

    if !assistant.is_connected() {
        println!("connect failed");
        drop(assistant);
        pause();
        return Ok(());
    }

    assistant.append_task(
        StartUpTask::builder()
            .enable(true)
            .client_type("Official")
            .start_game_enabled(true)
            .account_name("123****4567")
            .build(),
    )?;

    assistant.append_task(
        FightTask::builder()
            .enable(true)
            .stage("1-7")
            .medicine(3)
            .times(5)
            .build(),
    )?;

    assistant.start()?;
    println!("should be running");
    pause();
    assistant.stop()?;
    drop(assistant);
    Ok(())
}
