use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("资源加载失败")]
    ResourceLoadFailed,
    #[error("创建实例失败")]
    CreateFailed,
    #[error("连接失败")]
    ConnectFailed,
    #[error("任务添加失败")]
    TaskAppendFailed,
    #[error("任务参数设置失败")]
    TaskParamsSetFailed,
    #[error("启动失败")]
    StartFailed,
    #[error("停止失败")]
    StopFailed,
    #[error("返回主页失败")]
    BackToHomeFailed,
    #[error("点击失败")]
    ClickFailed,
    #[error("截图失败")]
    CaptureFailed,
    #[error("设置实例选项失败")]
    SetInstanceOptionFailed,
    #[error("设置选项失败")]
    SetStaticOptionFailed,
    #[error("未知错误")]
    Unknown,
}

#[repr(u8)]
#[derive(Clone, Copy)]
pub enum StaticOptionKey {
    /// 无效
    Invalid,
    /// 用CPU进行OCR
    CpuOCR,
    /// 用GPU进行OCR
    GpuOCR,
}

#[repr(u8)]
#[derive(Clone, Copy)]
pub enum InstanceOptionKey {
    /// 已弃用
    Invalid = 0,
    /// 是否启用 minitouch
    /// 开了也不代表就一定能用，有可能设备不支持等
    /// "1" 开，"0" 关
    MinitouchEnabled = 1,
    /// 触控模式设置，默认 minitouch
    /// minitouch | maatouch | adb
    TouchMode = 2,
    /// 是否暂停下干员，同时影响抄作业、肉鸽、保全
    /// "1" | "0"
    DeploymentWithPause = 3,
    /// 是否使用 AdbLite， "0" | "1"
    AdbLiteEnabled = 4,
    /// 退出时是否杀掉 Adb 进程， "0" | "1"
    KillAdbOnExit = 5,
}
