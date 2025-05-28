use std::collections::HashMap;

use maa_sys_derive::Task;
use serde::{Deserialize, Serialize};
use serde_json;
use serde_with::skip_serializing_none;

pub trait Task {
    fn task_type(&self) -> &'static str;
    fn to_json(&self) -> String;
}

/// 开始唤醒任务的参数
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Task)]
pub struct StartUpTask {
    /// 是否启用本任务，可选，默认为 true
    pub enable: Option<bool>,
    /// 客户端版本，可选，默认为空
    /// 选项："Official" | "Bilibili" | "txwy" | "YoStarEN" | "YoStarJP" | "YoStarKR"
    pub client_type: Option<String>,
    /// 是否自动启动客户端，可选，默认不启动
    pub start_game_enabled: Option<bool>,
    /// 切换账号，可选，默认不切换
    /// 仅支持切换至已登录的账号，使用登录名进行查找，保证输入内容在所有已登录账号唯一即可
    /// 官服：123****4567，可输入 123****4567、4567、123、3****4567
    /// B服：张三，可输入 张三、张、三
    pub account_name: Option<String>,
}

/// 关闭游戏任务的参数
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Task)]
pub struct CloseDownTask {
    /// 是否启用本任务，可选，预设为 true
    pub enable: Option<bool>,
    /// 客户端版本，必选，填空则不执行
    /// 选项："Official" | "Bilibili" | "txwy" | "YoStarEN" | "YoStarJP" | "YoStarKR"
    pub client_type: Option<String>,
}

/// 刷理智任务的参数
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Task)]
pub struct FightTask {
    /// 是否启用本任务，可选，默认为 true
    pub enable: Option<bool>,
    /// 关卡名，可选，默认为空，识别当前/上次的关卡。不支持运行中设置
    pub stage: Option<String>,
    /// 最大使用理智药数量，可选，默认 0
    pub medicine: Option<i32>,
    /// 最大使用 48 小时内过期理智药数量，可选，默认 0
    pub expiring_medicine: Option<i32>,
    /// 最大吃石头数量，可选，默认 0
    pub stone: Option<i32>,
    /// 战斗次数，可选，默认int32.max
    pub times: Option<i32>,
    /// 连战次数, 可选, -1~6
    pub series: Option<i32>,
    /// 指定掉落数量，可选，默认不指定
    pub drops: Option<HashMap<String, i32>>,
    /// 是否汇报企鹅数据，可选，默认 false
    pub report_to_penguin: Option<bool>,
    /// 企鹅数据汇报 id, 可选，默认为空
    pub penguin_id: Option<String>,
    /// 服务器，可选，默认 "CN"
    pub server: Option<String>,
    /// 客户端版本，可选，默认为空
    pub client_type: Option<String>,
    /// 节省理智碎石模式，可选，默认 false
    pub dr_grandet: Option<bool>,
}

/// 公开招募任务的参数
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Task)]
pub struct RecruitTask {
    /// 是否启用本任务，可选，默认为 true
    pub enable: Option<bool>,
    /// 是否刷新三星 Tags, 可选，默认 false
    pub refresh: Option<bool>,
    /// 会去点击标签的 Tag 等级，必选
    pub select: Vec<i32>,
    /// 会去点击确认的 Tag 等级，必选
    pub confirm: Vec<i32>,
    /// 首选 Tags，仅在 Tag 等级为 3 时有效
    pub first_tags: Option<Vec<String>>,
    /// 选择更多的 Tags, 可选, 默认为 0
    pub extra_tags_mode: Option<i32>,
    /// 招募多少次，可选，默认 0
    pub times: Option<i32>,
    /// 是否设置招募时限
    pub set_time: Option<bool>,
    /// 是否使用加急许可，可选，默认 false
    pub expedite: Option<bool>,
    /// 加急次数，仅在 expedite 为 true 时有效
    pub expedite_times: Option<i32>,
    /// 是否在识别到小车词条时跳过，可选，默认跳过
    pub skip_robot: Option<bool>,
    /// Tag 等级和对应的希望招募时限
    pub recruitment_time: Option<HashMap<String, i32>>,
    /// 是否汇报企鹅数据，可选，默认 false
    pub report_to_penguin: Option<bool>,
    /// 企鹅数据汇报 id
    pub penguin_id: Option<String>,
    /// 是否汇报一图流数据，可选，默认 false
    pub report_to_yituliu: Option<bool>,
    /// 一图流汇报 id
    pub yituliu_id: Option<String>,
    /// 服务器，可选，默认 "CN"
    pub server: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_start_up_task_builder() {
        let task = StartUpTask::builder()
            .enable(true)
            .client_type("Official")
            .start_game_enabled(true)
            .account_name("123****4567")
            .build();

        assert_eq!(Some(true), task.enable, "task.enable");
        assert_eq!(Some("Official".to_string()), task.client_type, "task.client_type");
        assert_eq!(Some(true), task.start_game_enabled, "task.start_game_enabled");
        assert_eq!(
            Some("123****4567".to_string()),
            task.account_name,
            "task.account_name"
        );

        assert_eq!(
            r#"{"enable":true,"client_type":"Official","start_game_enabled":true,"account_name":"123****4567"}"#,
            task.to_json(),
            "task.to_json"
        );

        // 测试默认值
        let default_task = StartUpTask::builder().build();
        assert_eq!(None, default_task.enable, "default_task.enable");
        assert_eq!(None, default_task.client_type, "default_task.client_type");
        assert_eq!(
            None, default_task.start_game_enabled,
            "default_task.start_game_enabled"
        );
        assert_eq!(None, default_task.account_name, "default_task.account_name");

        assert_eq!(r#"{}"#, default_task.to_json(), "default_task.to_json");
    }

    #[test]
    fn test_recruit_task_builder() {
        let task = RecruitTask::builder()
            .enable(true)
            .select(vec![1, 2, 3])
            .confirm(vec![4, 5, 6])
            .build();

        assert_eq!(Some(true), task.enable, "task.enable");
        
        assert_eq!(
            r#"{"enable":true,"select":[1,2,3],"confirm":[4,5,6]}"#,
            task.to_json(),
            "task.to_json"
        );

        // 测试必选字段未设置的情况
        let result = std::panic::catch_unwind(|| {
            RecruitTask::builder().enable(true).build();
        });
        assert!(result.is_err(), "必选字段未设置应该 panic");
    }
}
