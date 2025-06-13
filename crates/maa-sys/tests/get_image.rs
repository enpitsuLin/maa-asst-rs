use maa_sys::{Connection, InstanceOptionKey};

mod common;

#[test]
fn test_get_image() {
    let mut assistant = common::create_test_assistant();

    assistant
        .set_instance_option(InstanceOptionKey::TouchMode, "adb")
        .unwrap();

    assistant
        .connect(Connection::adb("adb", "192.168.20.29:44847"), None)
        .unwrap();

    assistant.capture_screenshot().unwrap();

    let image = assistant.get_image().unwrap();

    assert!(!image.is_empty());
}
