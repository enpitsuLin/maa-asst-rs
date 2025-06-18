use std::env;
use std::path::PathBuf;

/// 获取测试资源路径
pub fn get_test_resource_path() -> PathBuf {
    let resource_path = env::var("MAA_RESOURCE_PATH").expect("MAA_RESOURCE_PATH 环境变量未设置");
    PathBuf::from(resource_path)
}

/// 创建测试用的 Assistant 实例
pub fn create_test_assistant() -> maa_sys::Assistant {
    maa_sys::Assistant::init(get_test_resource_path()).expect("创建 Assistant 实例失败")
}
