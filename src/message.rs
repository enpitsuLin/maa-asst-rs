use std::convert::TryFrom;

/// 回调消息类型枚举
#[derive(PartialEq, Debug)]
#[allow(dead_code)]
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

impl TryFrom<i32> for AsstMsg {
    type Error = ();

    fn try_from(input: i32) -> Result<Self, Self::Error> {
        let input = input.to_owned();
        match input {
            x if x == AsstMsg::InternalError as i32 => Ok(AsstMsg::InternalError),
            x if x == AsstMsg::InitFailed as i32 => Ok(AsstMsg::InitFailed),
            x if x == AsstMsg::ConnectionInfo as i32 => Ok(AsstMsg::ConnectionInfo),
            x if x == AsstMsg::AllTasksCompleted as i32 => Ok(AsstMsg::AllTasksCompleted),
            x if x == AsstMsg::TaskChainError as i32 => Ok(AsstMsg::TaskChainError),
            x if x == AsstMsg::TaskChainStart as i32 => Ok(AsstMsg::TaskChainStart),
            x if x == AsstMsg::TaskChainCompleted as i32 => Ok(AsstMsg::TaskChainCompleted),
            x if x == AsstMsg::TaskChainExtraInfo as i32 => Ok(AsstMsg::TaskChainExtraInfo),
            x if x == AsstMsg::SubTaskError as i32 => Ok(AsstMsg::SubTaskError),
            x if x == AsstMsg::SubTaskStart as i32 => Ok(AsstMsg::SubTaskStart),
            x if x == AsstMsg::SubTaskCompleted as i32 => Ok(AsstMsg::SubTaskCompleted),
            x if x == AsstMsg::SubTaskExtraInfo as i32 => Ok(AsstMsg::SubTaskExtraInfo),
            _ => Err(()),
        }
    }
}

#[test]
fn test_covert() {
    let input = AsstMsg::try_from(0).unwrap();
    assert_eq!(input, AsstMsg::InternalError)
}
