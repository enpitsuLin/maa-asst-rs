use std::env;
use std::path::PathBuf;

/// 获取测试资源路径
pub fn get_test_resource_path() -> PathBuf {
    let resource_path = env::var("MAA_RESOURCE_PATH").expect("MAA_RESOURCE_PATH 环境变量未设置");
    PathBuf::from(resource_path)
}

/// 创建测试用的 Assistant 实例
pub fn create_test_assistant() -> maa_sys::Assistant {
    maa_sys::Assistant::new(get_test_resource_path()).expect("创建 Assistant 实例失败")
}

/// 等待助手完成操作
pub fn wait_for_assistant(assistant: &maa_sys::Assistant, timeout_secs: u64) -> bool {
    let start = std::time::Instant::now();
    while assistant.is_running() {
        if start.elapsed().as_secs() > timeout_secs {
            return false;
        }
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
    true
}
