/// 回调消息类型枚举
pub enum AsstMsg {
    /* Global Info */
    /// 内部错误
    InternalError = 0,
    /// 初始化失败
    InitFailed = 1,
    /// 连接相关信息
    ConnectionInfo = 2,
    /// 全部任务完成
    AllTasksCompleted = 3,
    /* TaskChain Info */
    /// 任务链执行/识别错误
    TaskChainError = 10000,
    /// 任务链开始
    TaskChainStart = 10001,
    /// 任务链完成
    TaskChainCompleted = 10002,
    /// 任务链额外信息
    TaskChainExtraInfo = 10003,
    /* SubTask Info */
    /// 原子任务执行/识别错误
    SubTaskError = 20000,
    /// 原子任务开始
    SubTaskStart = 20001,
    /// 原子任务完成
    SubTaskCompleted = 20002,
    /// 原子任务额外信息
    SubTaskExtraInfo = 20003,
}
