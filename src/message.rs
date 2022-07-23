/// 回调消息类型枚举
#[derive(PartialEq)]
pub enum AsstMsg {
    /* Global Info */
    /// 内部错误
    InternalError = 0,
    /// 初始化失败
    InitFailed,
    /// 连接相关信息
    ConnectionInfo,
    /// 全部任务完成
    AllTasksCompleted,
    /* TaskChain Info */
    /// 任务链执行/识别错误
    TaskChainError = 10000,
    /// 任务链开始
    TaskChainStart,
    /// 任务链完成
    TaskChainCompleted,
    /// 任务链额外信息
    TaskChainExtraInfo,
    /* SubTask Info */
    /// 原子任务执行/识别错误
    SubTaskError = 20000,
    /// 原子任务开始
    SubTaskStart,
    /// 原子任务完成
    SubTaskCompleted,
    /// 原子任务额外信息
    SubTaskExtraInfo,
}
