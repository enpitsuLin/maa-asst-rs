use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AsstMsg {
    /* Global Info */
    InternalError = 0,     // 内部错误
    InitFailed = 1,        // 初始化失败
    ConnectionInfo = 2,    // 连接相关信息
    AllTasksCompleted = 3, // 全部任务完成
    AsyncCallInfo = 4,     // 外部异步调用信息
    Destroyed = 5,         // 实例已销毁

    /* TaskChain Info */
    TaskChainError = 10000,     // 任务链执行/识别错误
    TaskChainStart = 10001,     // 任务链开始
    TaskChainCompleted = 10002, // 任务链完成
    TaskChainExtraInfo = 10003, // 任务链额外信息
    TaskChainStopped = 10004,   // 任务链手动停止

    /* SubTask Info */
    SubTaskError = 20000,     // 原子任务执行/识别错误
    SubTaskStart = 20001,     // 原子任务开始
    SubTaskCompleted = 20002, // 原子任务完成
    SubTaskExtraInfo = 20003, // 原子任务额外信息
    SubTaskStopped = 20004,   // 原子任务手动停止
}

impl Display for AsstMsg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// 回调处理器
///
/// 因为 Rust 会对零大小的分配优化，直接使用 Rust 的函数类型会导致分配失败，指针永远只能得到 `0x1`
/// 所以需要使用一个非空结构体来包装回调函数
pub struct Processor {
    pub callback: Box<dyn FnMut(AsstMsg, serde_json::Value) + Send>,
}

impl Processor {
    pub fn from(callback: impl FnMut(AsstMsg, serde_json::Value) + Send + 'static) -> Self {
        Self {
            callback: Box::new(callback),
        }
    }
}

impl From<i32> for AsstMsg {
    fn from(value: i32) -> Self {
        match value {
            0 => AsstMsg::InternalError,
            1 => AsstMsg::InitFailed,
            2 => AsstMsg::ConnectionInfo,
            3 => AsstMsg::AllTasksCompleted,
            4 => AsstMsg::AsyncCallInfo,
            5 => AsstMsg::Destroyed,
            10000 => AsstMsg::TaskChainError,
            10001 => AsstMsg::TaskChainStart,
            10002 => AsstMsg::TaskChainCompleted,
            10003 => AsstMsg::TaskChainExtraInfo,
            10004 => AsstMsg::TaskChainStopped,
            20000 => AsstMsg::SubTaskError,
            20001 => AsstMsg::SubTaskStart,
            20002 => AsstMsg::SubTaskCompleted,
            20003 => AsstMsg::SubTaskExtraInfo,
            20004 => AsstMsg::SubTaskStopped,
            _ => AsstMsg::InternalError, // 对于未知的消息ID，返回内部错误
        }
    }
}
