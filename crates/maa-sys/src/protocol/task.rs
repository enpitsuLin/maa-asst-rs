use hashbrown::HashMap;
use maa_macros::GenerateTask;
use serde::{Deserialize, Serialize};
use serde_json;
use serde_with::skip_serializing_none;

pub trait Task {
    fn task_type(&self) -> &'static str;
    fn task_name(&self) -> &'static str;
    fn to_json(&self) -> String;
    fn from_json(json: &str) -> Result<Self, serde_json::Error>
    where
        Self: Sized;
}

/// 开始唤醒任务的参数
///
/// # 字段说明
///
/// * `enable` - 是否启用本任务，默认为 `true`
/// * `client_type` - 客户端版本，可选值：
///   - "Official" - 官服
///   - "Bilibili" - B服
///   - "txwy" - 腾讯服
///   - "YoStarEN" - 国际服
///   - "YoStarJP" - 日服
///   - "YoStarKR" - 韩服
/// * `start_game_enabled` - 是否自动启动客户端，默认为 `false`
/// * `account_name` - 切换账号，仅支持切换至已登录的账号
///
/// # 示例
///
/// ```
/// use maa_sys::task::StartUpTask;
///
/// let task = StartUpTask::builder()
///     .enable(true)
///     .client_type("Official")
///     .start_game_enabled(true)
///     .account_name("123****4567")
///     .build();
/// ```
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, GenerateTask)]
#[task(name = "开始唤醒", task_type = "StartUp")]
pub struct StartUpTask {
    /// 是否启用本任务，默认为 `true`
    pub enable: Option<bool>,
    /// 客户端版本，可选值：
    /// - "Official" - 官服
    /// - "Bilibili" - B服
    /// - "txwy" - 腾讯服
    /// - "YoStarEN" - 国际服
    /// - "YoStarJP" - 日服
    /// - "YoStarKR" - 韩服
    pub client_type: Option<String>,
    /// 是否自动启动客户端，默认为 `false`
    pub start_game_enabled: Option<bool>,
    /// 切换账号，仅支持切换至已登录的账号
    ///
    /// 使用登录名进行查找，保证输入内容在所有已登录账号唯一即可：
    /// - 官服：123****4567，可输入 123****4567、4567、123、3****4567
    /// - B服：张三，可输入 张三、张、三
    pub account_name: Option<String>
}

/// 关闭游戏任务的参数
///
/// # 字段说明
///
/// * `enable` - 是否启用本任务，默认为 `true`
/// * `client_type` - 客户端版本，必选，填空则不执行
///
/// # 示例
///
/// ```
/// use maa_sys::task::CloseDownTask;
///
/// let task = CloseDownTask::builder()
///     .enable(true)
///     .client_type("Official")
///     .build();
/// ```
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, GenerateTask)]
#[task(name = "关闭游戏", task_type = "CloseDown")]
pub struct CloseDownTask {
    /// 是否启用本任务，默认为 `true`
    pub enable: Option<bool>,
    /// 客户端版本，必选，填空则不执行
    ///
    /// 可选值：
    /// - "Official" - 官服
    /// - "Bilibili" - B服
    /// - "txwy" - 腾讯服
    /// - "YoStarEN" - 国际服
    /// - "YoStarJP" - 日服
    /// - "YoStarKR" - 韩服
    pub client_type: Option<String>
}

/// 刷理智任务的参数
///
/// # 字段说明
///
/// * `enable` - 是否启用本任务，默认为 `true`
/// * `stage` - 关卡名，默认为空，识别当前/上次的关卡。不支持运行中设置
/// * `medicine` - 最大使用理智药数量，默认为 `0`
/// * `expiring_medicine` - 最大使用 48 小时内过期理智药数量，默认为 `0`
/// * `stone` - 最大吃石头数量，默认为 `0`
/// * `times` - 战斗次数，默认为 `i32::MAX`
/// * `series` - 连战次数，取值范围 `-1~6`
/// * `drops` - 指定掉落数量，默认为不指定
/// * `report_to_penguin` - 是否汇报企鹅数据，默认为 `false`
/// * `penguin_id` - 企鹅数据汇报 id
/// * `server` - 服务器，默认为 "CN"
/// * `client_type` - 客户端版本
/// * `dr_grandet` - 节省理智碎石模式，默认为 `false`
///
/// # 示例
///
/// ```
/// use maa_sys::task::FightTask;
/// use std::collections::HashMap;
///
/// let mut drops = HashMap::new();
/// drops.insert("龙门币".to_string(), 1000);
///
/// let task = FightTask::builder()
///     .enable(true)
///     .stage("1-7")
///     .medicine(5)
///     .times(10)
///     .drops(drops)
///     .build();
/// ```
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, GenerateTask)]
#[task(name = "刷理智", task_type = "Fight")]
pub struct FightTask {
    /// 是否启用本任务，默认为 `true`
    pub enable: Option<bool>,
    /// 关卡名，默认为空，识别当前/上次的关卡。不支持运行中设置
    pub stage: Option<String>,
    /// 最大使用理智药数量，默认为 `0`
    pub medicine: Option<i32>,
    /// 最大使用 48 小时内过期理智药数量，默认为 `0`
    pub expiring_medicine: Option<i32>,
    /// 最大吃石头数量，默认为 `0`
    pub stone: Option<i32>,
    /// 战斗次数，默认为 `i32::MAX`
    pub times: Option<i32>,
    /// 连战次数，取值范围 `-1~6`
    pub series: Option<i32>,
    /// 指定掉落数量，默认为不指定
    pub drops: Option<HashMap<String, i32>>,
    /// 是否汇报企鹅数据，默认为 `false`
    pub report_to_penguin: Option<bool>,
    /// 企鹅数据汇报 id
    pub penguin_id: Option<String>,
    /// 服务器，默认为 "CN"
    pub server: Option<String>,
    /// 客户端版本
    pub client_type: Option<String>,
    /// 节省理智碎石模式，默认为 `false`
    pub dr_grandet: Option<bool>
}

/// 公开招募任务的参数
///
/// # 字段说明
///
/// * `enable` - 是否启用本任务，默认为 `true`
/// * `refresh` - 是否刷新三星 Tags，默认为 `false`
/// * `select` - 会去点击标签的 Tag 等级，必选
/// * `confirm` - 会去点击确认的 Tag 等级，必选
/// * `first_tags` - 首选 Tags，仅在 Tag 等级为 3 时有效
/// * `extra_tags_mode` - 选择更多的 Tags，默认为 `0`
/// * `times` - 招募多少次，默认为 `0`
/// * `set_time` - 是否设置招募时限
/// * `expedite` - 是否使用加急许可，默认为 `false`
/// * `expedite_times` - 加急次数，仅在 expedite 为 true 时有效
/// * `skip_robot` - 是否在识别到小车词条时跳过，默认为跳过
/// * `recruitment_time` - Tag 等级和对应的希望招募时限
/// * `report_to_penguin` - 是否汇报企鹅数据，默认为 `false`
/// * `penguin_id` - 企鹅数据汇报 id
/// * `report_to_yituliu` - 是否汇报一图流数据，默认为 `false`
/// * `yituliu_id` - 一图流汇报 id
/// * `server` - 服务器，默认为 "CN"
///
/// # 示例
///
/// ```
/// use maa_sys::task::RecruitTask;
///
/// let task = RecruitTask::builder()
///     .enable(true)
///     .select(vec![1, 2, 3])
///     .confirm(vec![4, 5, 6])
///     .build();
/// ```
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, GenerateTask)]
#[task(name = "公开招募", task_type = "Recruit")]
pub struct RecruitTask {
    /// 是否启用本任务，默认为 `true`
    pub enable: Option<bool>,
    /// 是否刷新三星 Tags，默认为 `false`
    pub refresh: Option<bool>,
    /// 会去点击标签的 Tag 等级，必选
    pub select: Vec<i32>,
    /// 会去点击确认的 Tag 等级，必选
    pub confirm: Vec<i32>,
    /// 首选 Tags，仅在 Tag 等级为 3 时有效
    pub first_tags: Option<Vec<String>>,
    /// 选择更多的 Tags，默认为 `0`
    pub extra_tags_mode: Option<i32>,
    /// 招募多少次，默认为 `0`
    pub times: Option<i32>,
    /// 是否设置招募时限
    pub set_time: Option<bool>,
    /// 是否使用加急许可，默认为 `false`
    pub expedite: Option<bool>,
    /// 加急次数，仅在 expedite 为 true 时有效
    pub expedite_times: Option<i32>,
    /// 是否在识别到小车词条时跳过，默认为跳过
    pub skip_robot: Option<bool>,
    /// Tag 等级和对应的希望招募时限
    pub recruitment_time: Option<HashMap<String, i32>>,
    /// 是否汇报企鹅数据，默认为 `false`
    pub report_to_penguin: Option<bool>,
    /// 企鹅数据汇报 id
    pub penguin_id: Option<String>,
    /// 是否汇报一图流数据，默认为 `false`
    pub report_to_yituliu: Option<bool>,
    /// 一图流汇报 id
    pub yituliu_id: Option<String>,
    /// 服务器，默认为 "CN"
    pub server: Option<String>
}

// Start Generation Here
/// 基建换班任务的参数
///
/// # 字段说明
///
/// * `enable` - 是否启用本任务，默认为 `true`
/// * `mode` - 换班工作模式，默认为 `0`
///   - `0` - Default: 默认换班模式，单设施最优解
///   - `10000` - Custom: 自定义换班模式，读取用户配置
///   - `20000` - Rotation: 一键轮换模式
/// * `facility` - 要换班的设施（有序），必选。不支持运行中设置
/// * `drones` - 无人机用途，默认为 _NotUse
/// * `threshold` - 工作心情阈值，取值范围 [0, 1.0]，默认为 0.3
/// * `replenish` - 贸易站"源石碎片"是否自动补货，默认为 `false`
/// * `dorm_not_stationed_enabled` - 是否启用宿舍"未进驻"选项，默认为 `false`
/// * `dorm_trust_enabled` - 是否将宿舍剩余位置填入信赖未满干员，默认为 `false`
/// * `reception_message_board` - 是否领取会客室信息板信用，默认为 `true`
/// * `filename` - 自定义配置路径，必选。不支持运行中设置
/// * `plan_index` - 使用配置中的方案序号，必选。不支持运行中设置
///
/// # 示例
///
/// ```
/// use maa_sys::task::InfrastTask;
///
/// let task = InfrastTask::builder()
///     .enable(true)
///     .mode(0)
///     .facility(vec!["制造站".to_string(), "贸易站".to_string()])
///     .threshold(0.3)
///     .build();
/// ```
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, GenerateTask)]
#[task(name = "基建换班", task_type = "Infrast")]
pub struct InfrastTask {
    /// 是否启用本任务，默认为 `true`
    pub enable: Option<bool>,
    /// 换班工作模式，默认为 `0`
    ///
    /// 可选值：
    /// - `0` - Default: 默认换班模式，单设施最优解
    /// - `10000` - Custom: 自定义换班模式，读取用户配置
    /// - `20000` - Rotation: 一键轮换模式
    pub mode: Option<i32>,
    /// 要换班的设施（有序），必选。不支持运行中设置
    pub facility: Vec<String>,
    /// 无人机用途，默认为 _NotUse
    pub drones: Option<String>,
    /// 工作心情阈值，取值范围 [0, 1.0]，默认为 0.3
    pub threshold: Option<f32>,
    /// 贸易站"源石碎片"是否自动补货，默认为 `false`
    pub replenish: Option<bool>,
    /// 是否启用宿舍"未进驻"选项，默认为 `false`
    pub dorm_not_stationed_enabled: Option<bool>,
    /// 是否将宿舍剩余位置填入信赖未满干员，默认为 `false`
    pub dorm_trust_enabled: Option<bool>,
    /// 是否领取会客室信息板信用，默认为 `true`
    pub reception_message_board: Option<bool>,
    /// 自定义配置路径，必选。不支持运行中设置
    pub filename: Option<String>,
    /// 使用配置中的方案序号，必选。不支持运行中设置
    pub plan_index: Option<i32>
}

/// 商店任务的参数
///
/// # 字段说明
///
/// * `enable` - 是否启用本任务，默认为 `true`
/// * `shopping` - 是否购物，默认为 `false`。不支持运行中设置
/// * `buy_first` - 优先购买列表。不支持运行中设置
/// * `blacklist` - 黑名单列表。不支持运行中设置
/// * `force_shopping_if_credit_full` - 是否在信用溢出时无视黑名单，默认为 `true`
/// * `only_buy_discount` - 是否只购买折扣物品，只作用于第二轮购买，默认为 `false`
/// * `reserve_max_credit` - 是否在信用点低于300时停止购买，只作用于第二轮购买，默认为 `false`
///
/// # 示例
///
/// ```
/// use maa_sys::task::MallTask;
///
/// let task = MallTask::builder()
///     .enable(true)
///     .shopping(true)
///     .buy_first(vec!["龙门币".to_string()])
///     .build();
/// ```
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, GenerateTask)]
#[task(name = "商店", task_type = "Mall")]
pub struct MallTask {
    /// 是否启用本任务，默认为 `true`
    pub enable: Option<bool>,
    /// 是否购物，默认为 `false`。不支持运行中设置
    pub shopping: Option<bool>,
    /// 优先购买列表。不支持运行中设置
    pub buy_first: Option<Vec<String>>,
    /// 黑名单列表。不支持运行中设置
    pub blacklist: Option<Vec<String>>,
    /// 是否在信用溢出时无视黑名单，默认为 `true`
    pub force_shopping_if_credit_full: Option<bool>,
    /// 是否只购买折扣物品，只作用于第二轮购买，默认为 `false`
    pub only_buy_discount: Option<bool>,
    /// 是否在信用点低于300时停止购买，只作用于第二轮购买，默认为 `false`
    pub reserve_max_credit: Option<bool>
}

/// 奖励领取任务的参数
///
/// # 字段说明
///
/// * `enable` - 是否启用本任务，默认为 `true`
/// * `award` - 领取每日/每周任务奖励，默认为 `true`
/// * `mail` - 领取所有邮件奖励，默认为 `false`
/// * `recruit` - 领取限定池子赠送的每日免费单抽，默认为 `false`
/// * `orundum` - 领取幸运墙的合成玉奖励，默认为 `false`
/// * `mining` - 领取限时开采许可的合成玉奖励，默认为 `false`
/// * `specialaccess` - 领取五周年赠送的月卡奖励，默认为 `false`
///
/// # 示例
///
/// ```
/// use maa_sys::task::AwardTask;
///
/// let task = AwardTask::builder()
///     .enable(true)
///     .award(true)
///     .mail(true)
///     .build();
/// ```
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, GenerateTask)]
#[task(name = "奖励领取", task_type = "Award")]
pub struct AwardTask {
    /// 是否启用本任务，默认为 `true`
    pub enable: Option<bool>,
    /// 领取每日/每周任务奖励，默认为 `true`
    pub award: Option<bool>,
    /// 领取所有邮件奖励，默认为 `false`
    pub mail: Option<bool>,
    /// 领取限定池子赠送的每日免费单抽，默认为 `false`
    pub recruit: Option<bool>,
    /// 领取幸运墙的合成玉奖励，默认为 `false`
    pub orundum: Option<bool>,
    /// 领取限时开采许可的合成玉奖励，默认为 `false`
    pub mining: Option<bool>,
    /// 领取五周年赠送的月卡奖励，默认为 `false`
    pub specialaccess: Option<bool>
}

/// 肉鸽任务的参数
///
/// # 字段说明
///
/// * `enable` - 是否启用本任务，默认为 `true`
/// * `theme` - 主题，默认为 "Phantom"
/// * `mode` - 模式，默认为 `0`
/// * `squad` - 开局分队名，默认为 "指挥分队"
/// * `roles` - 开局职业组，默认为 "取长补短"
/// * `core_char` - 开局干员名
/// * `use_support` - 开局干员是否为助战干员，默认为 `false`
/// * `use_nonfriend_support` - 是否可以是非好友助战干员，默认为 `false`
/// * `starts_count` - 开始探索的次数，默认为 `i32::MAX`
/// * `difficulty` - 指定难度等级，默认为 `0`
/// * `stop_at_final_boss` - 是否在第 5 层险路恶敌节点前停止任务，默认为 `false`
/// * `stop_at_max_level` - 是否在肉鸽等级刷满后停止任务，默认为 `false`
/// * `investment_enabled` - 是否投资源石锭，默认为 `true`
/// * `investments_count` - 投资源石锭的次数，默认为 `i32::MAX`
/// * `stop_when_investment_full` - 是否在投资到达上限后自动停止任务，默认为 `false`
/// * `investment_with_more_score` - 是否在投资后尝试购物，默认为 `false`
/// * `start_with_elite_two` - 是否在凹开局的同时凹干员精二直升，默认为 `false`
/// * `only_start_with_elite_two` - 是否只凹开局干员精二直升而忽视其他开局条件，默认为 `false`
/// * `refresh_trader_with_dice` - 是否用骰子刷新商店购买特殊商品，默认为 `false`
/// * `first_floor_foldartal` - 希望在第一层远见阶段得到的密文版
/// * `start_foldartal_list` - 凹开局时希望在开局奖励阶段得到的密文板，默认为 `[]`
/// * `start_with_two_ideas` - 是否凹 2 构想开局，默认为 `false`
/// * `use_foldartal` - 是否使用密文板，模式 5 下默认为 `false`，其他模式下默认为 `true`
/// * `check_collapsal_paradigms` - 是否检测获取的坍缩范式，模式 5 下默认为 `true`，其他模式下默认为 `false`
/// * `double_check_collapsal_paradigms` - 是否执行坍缩范式检测防漏措施，模式 5 下默认为 `true`，其他模式下默认为 `false`
/// * `expected_collapsal_paradigms` - 希望触发的坍缩范式，默认为 `["目空一些", "睁眼瞎", "图像损坏", "一抹黑"]`
/// * `monthly_squad_auto_iterate` - 是否启动月度小队自动切换
/// * `monthly_squad_check_comms` - 是否将月度小队通信也作为切换依据
/// * `deep_exploration_auto_iterate` - 是否启动深入调查自动切换
/// * `collectible_mode_shopping` - 烧水是否启用购物，默认为 `false`
/// * `collectible_mode_squad` - 烧水时使用的分队，默认与 squad 同步
/// * `collectible_mode_start_list` - 烧水期望奖励，默认全 `false`
/// * `start_with_seed` - 使用种子刷钱，`true` 时有效
///
/// # 示例
///
/// ```
/// use maa_sys::task::RoguelikeTask;
///
/// let task = RoguelikeTask::builder()
///     .enable(true)
///     .theme("Phantom")
///     .mode(0)
///     .squad("指挥分队")
///     .build();
/// ```
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, GenerateTask)]
#[task(name = "肉鸽", task_type = "Roguelike")]
pub struct RoguelikeTask {
    /// 是否启用本任务，默认为 `true`
    pub enable: Option<bool>,
    /// 主题，默认为 "Phantom"
    pub theme: Option<String>,
    /// 模式，默认为 `0`
    pub mode: Option<i32>,
    /// 开局分队名，默认为 "指挥分队"
    pub squad: Option<String>,
    /// 开局职业组，默认为 "取长补短"
    pub roles: Option<String>,
    /// 开局干员名
    pub core_char: Option<String>,
    /// 开局干员是否为助战干员，默认为 `false`
    pub use_support: Option<bool>,
    /// 是否可以是非好友助战干员，默认为 `false`
    pub use_nonfriend_support: Option<bool>,
    /// 开始探索的次数，默认为 `i32::MAX`
    pub starts_count: Option<i32>,
    /// 指定难度等级，默认为 `0`
    pub difficulty: Option<i32>,
    /// 是否在第 5 层险路恶敌节点前停止任务，默认为 `false`
    pub stop_at_final_boss: Option<bool>,
    /// 是否在肉鸽等级刷满后停止任务，默认为 `false`
    pub stop_at_max_level: Option<bool>,
    /// 是否投资源石锭，默认为 `true`
    pub investment_enabled: Option<bool>,
    /// 投资源石锭的次数，默认为 `i32::MAX`
    pub investments_count: Option<i32>,
    /// 是否在投资到达上限后自动停止任务，默认为 `false`
    pub stop_when_investment_full: Option<bool>,
    /// 是否在投资后尝试购物，默认为 `false`
    pub investment_with_more_score: Option<bool>,
    /// 是否在凹开局的同时凹干员精二直升，默认为 `false`
    pub start_with_elite_two: Option<bool>,
    /// 是否只凹开局干员精二直升而忽视其他开局条件，默认为 `false`
    pub only_start_with_elite_two: Option<bool>,
    /// 是否用骰子刷新商店购买特殊商品，默认为 `false`
    pub refresh_trader_with_dice: Option<bool>,
    /// 希望在第一层远见阶段得到的密文版
    pub first_floor_foldartal: Option<String>,
    /// 凹开局时希望在开局奖励阶段得到的密文板，默认为 `[]`
    pub start_foldartal_list: Option<Vec<String>>,
    /// 是否凹 2 构想开局，默认为 `false`
    pub start_with_two_ideas: Option<bool>,
    /// 是否使用密文板，模式 5 下默认为 `false`，其他模式下默认为 `true`
    pub use_foldartal: Option<bool>,
    /// 是否检测获取的坍缩范式，模式 5 下默认为 `true`，其他模式下默认为 `false`
    pub check_collapsal_paradigms: Option<bool>,
    /// 是否执行坍缩范式检测防漏措施，模式 5 下默认为 `true`，其他模式下默认为 `false`
    pub double_check_collapsal_paradigms: Option<bool>,
    /// 希望触发的坍缩范式，默认为 `["目空一些", "睁眼瞎", "图像损坏", "一抹黑"]`
    pub expected_collapsal_paradigms: Option<Vec<String>>,
    /// 是否启动月度小队自动切换
    pub monthly_squad_auto_iterate: Option<bool>,
    /// 是否将月度小队通信也作为切换依据
    pub monthly_squad_check_comms: Option<bool>,
    /// 是否启动深入调查自动切换
    pub deep_exploration_auto_iterate: Option<bool>,
    /// 烧水是否启用购物，默认为 `false`
    pub collectible_mode_shopping: Option<bool>,
    /// 烧水时使用的分队，默认与 squad 同步
    pub collectible_mode_squad: Option<String>,
    /// 烧水期望奖励，默认全 `false`
    pub collectible_mode_start_list: Option<HashMap<String, bool>>,
    /// 使用种子刷钱，`true` 时有效
    pub start_with_seed: Option<bool>
}

/// 自动抄作业任务的参数
///
/// # 字段说明
///
/// * `enable` - 是否启用本任务，默认为 `true`
/// * `filename` - 作业 JSON 的文件路径，绝对、相对路径均可。不支持运行期设置
/// * `formation` - 是否进行"快捷编队"，默认为 `false`。不支持运行期设置
///
/// # 示例
///
/// ```
/// use maa_sys::task::CopilotTask;
///
/// let task = CopilotTask::builder()
///     .enable(true)
///     .filename("path/to/copilot.json")
///     .formation(true)
///     .build();
/// ```
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, GenerateTask)]
#[task(name = "自动抄作业", task_type = "Copilot")]
pub struct CopilotTask {
    /// 是否启用本任务，默认为 `true`
    pub enable: Option<bool>,
    /// 作业 JSON 的文件路径，绝对、相对路径均可。不支持运行期设置
    pub filename: Option<String>,
    /// 是否进行"快捷编队"，默认为 `false`。不支持运行期设置
    pub formation: Option<bool>
}

/// 自动抄保全作业任务的参数
///
/// # 字段说明
///
/// * `enable` - 是否启用本任务，默认为 `true`
/// * `filename` - 作业 JSON 的文件路径，绝对、相对路径均可。不支持运行期设置
/// * `loop_times` - 循环执行次数
///
/// # 示例
///
/// ```
/// use maa_sys::task::SSSCopilotTask;
///
/// let task = SSSCopilotTask::builder()
///     .enable(true)
///     .filename("path/to/sss_copilot.json")
///     .loop_times(3)
///     .build();
/// ```
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, GenerateTask)]
#[task(name = "自动抄保全作业", task_type = "SSSCopilot")]
pub struct SSSCopilotTask {
    /// 是否启用本任务，默认为 `true`
    pub enable: Option<bool>,
    /// 作业 JSON 的文件路径，绝对、相对路径均可。不支持运行期设置
    pub filename: Option<String>,
    /// 循环执行次数
    pub loop_times: Option<i32>
}

/// 仓库识别任务的参数
///
/// # 字段说明
///
/// * `enable` - 是否启用本任务，默认为 `true`
///
/// # 示例
///
/// ```
/// use maa_sys::task::DepotTask;
///
/// let task = DepotTask::builder()
///     .enable(true)
///     .build();
/// ```
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, GenerateTask)]
#[task(name = "仓库识别", task_type = "Depot")]
pub struct DepotTask {
    /// 是否启用本任务，默认为 `true`
    pub enable: Option<bool>
}

/// 干员 box 识别任务的参数
///
/// # 字段说明
///
/// * `enable` - 是否启用本任务，默认为 `true`
///
/// # 示例
///
/// ```
/// use maa_sys::task::OperBoxTask;
///
/// let task = OperBoxTask::builder()
///     .enable(true)
///     .build();
/// ```
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, GenerateTask)]
#[task(name = "干员 box 识别", task_type = "OperBox")]
pub struct OperBoxTask {
    /// 是否启用本任务，默认为 `true`
    pub enable: Option<bool>
}

/// 生息演算任务的参数
///
/// # 字段说明
///
/// * `enable` - 是否启用本任务，默认为 `true`
/// * `theme` - 主题，默认为 "Fire"
///   - "Fire" - *沙中之火*
///   - "Tales" - *沙洲遗闻*
/// * `mode` - 模式，默认为 `0`
///   - `0` - 刷分与建造点，进入战斗直接退出
///   - `1` - 沙中之火：刷赤金，联络员买水后基地锻造；
///           沙洲遗闻：自动制造物品并读档刷货币
/// * `tools_to_craft` - 自动制造的物品列表，默认为 `["荧光棒"]`
/// * `increment_mode` - 点击类型，默认为 `0`
///   - `0` - 连点
///   - `1` - 长按
/// * `num_craft_batches` - 单次最大制造轮数，默认为 `16`
///
/// # 示例
///
/// ```
/// use maa_sys::task::ReclamationTask;
///
/// let task = ReclamationTask::builder()
///     .enable(true)
///     .theme("Fire")
///     .mode(0)
///     .build();
/// ```
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, GenerateTask)]
#[task(name = "生息演算", task_type = "Reclamation")]
pub struct ReclamationTask {
    /// 是否启用本任务，默认为 `true`
    pub enable: Option<bool>,
    /// 主题，默认为 "Fire"
    ///
    /// 可选值：
    /// - "Fire" - *沙中之火*
    /// - "Tales" - *沙洲遗闻*
    pub theme: Option<String>,
    /// 模式，默认为 `0`
    ///
    /// 可选值：
    /// - `0` - 刷分与建造点，进入战斗直接退出
    /// - `1` - 沙中之火：刷赤金，联络员买水后基地锻造；
    ///         沙洲遗闻：自动制造物品并读档刷货币
    pub mode: Option<i32>,
    /// 自动制造的物品列表，默认为 `["荧光棒"]`
    pub tools_to_craft: Option<Vec<String>>,
    /// 点击类型，默认为 `0`
    ///
    /// 可选值：
    /// - `0` - 连点
    /// - `1` - 长按
    pub increment_mode: Option<i32>,
    /// 单次最大制造轮数，默认为 `16`
    pub num_craft_batches: Option<i32>
}

/// 自定义任务的参数
///
/// # 字段说明
///
/// * `enable` - 是否启用本任务，必选
/// * `task_names` - 执行数组中第一个匹配上的任务（及后续 next 等）
///   若想执行多个任务，可多次 append Custom task
///
/// # 示例
///
/// ```
/// use maa_sys::task::CustomTask;
///
/// let task = CustomTask::builder()
///     .enable(true)
///     .task_names(vec!["StartUp".to_string(), "Fight".to_string()])
///     .build();
/// ```
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, GenerateTask)]
#[task(name = "自定义", task_type = "Custom")]
pub struct CustomTask {
    /// 是否启用本任务，必选
    pub enable: bool,
    /// 执行数组中第一个匹配上的任务（及后续 next 等）
    /// 若想执行多个任务，可多次 append Custom task
    pub task_names: Vec<String>
}

/// 单步任务的参数
///
/// # 字段说明
///
/// * `enable` - 是否启用本任务，必选
/// * `task_type` - 任务类型，目前仅支持 "copilot"
/// * `subtask` - 子任务类型，必选
///   - "stage" 设置关卡名，需要 "details": { "stage": "xxxx" }
///   - "start" 开始作战，无 details
///   - "action": 单步作战操作，details 需为作战协议中的单个 action
/// * `details` - 任务详情，根据子任务类型不同而变化
///
/// # 示例
///
/// ```
/// use maa_sys::task::SingleStepTask;
/// use std::collections::HashMap;
/// use serde_json::json;
///
/// let mut details = HashMap::new();
/// details.insert("stage".to_string(), json!("1-7"));
///
/// let task = SingleStepTask::builder()
///     .enable(true)
///     .task_type("copilot")
///     .subtask("stage")
///     .details(details)
///     .build();
/// ```
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, GenerateTask)]
#[task(name = "单步", task_type = "SingleStep")]
pub struct SingleStepTask {
    /// 是否启用本任务，必选
    pub enable: bool,
    /// 任务类型，目前仅支持 "copilot"
    pub task_type: String,
    /// 子任务类型，必选
    ///
    /// 可选值：
    /// - "stage" 设置关卡名，需要 "details": { "stage": "xxxx" }
    /// - "start" 开始作战，无 details
    /// - "action": 单步作战操作，details 需为作战协议中的单个 action
    pub subtask: String,
    /// 任务详情，根据子任务类型不同而变化
    pub details: HashMap<String, serde_json::Value>
}

/// 视频识别任务的参数
///
/// # 字段说明
///
/// * `enable` - 是否启用本任务，必选
/// * `filename` - 视频的文件路径，绝对、相对路径均可。不支持运行期设置
///
/// # 示例
///
/// ```
/// use maa_sys::task::VideoRecognitionTask;
///
/// let task = VideoRecognitionTask::builder()
///     .enable(true)
///     .filename("path/to/video.mp4")
///     .build();
/// ```
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, GenerateTask)]
#[task(name = "视频识别", task_type = "VideoRecognition")]
pub struct VideoRecognitionTask {
    /// 是否启用本任务，必选
    pub enable: bool,
    /// 视频的文件路径，绝对、相对路径均可。不支持运行期设置
    pub filename: String
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

    #[test]
    fn test_from_json() {
        let json = r#"{"enable":true,"select":[1,2,3],"confirm":[4,5,6]}"#;
        let task = RecruitTask::from_json(json).unwrap();
        assert_eq!(Some(true), task.enable, "task.enable");
        assert_eq!(vec![1, 2, 3], task.select, "task.select");
        assert_eq!(vec![4, 5, 6], task.confirm, "task.confirm");
    }

    #[test]
    fn test_task_name() {
        assert_eq!("开始唤醒", StartUpTask::builder().build().task_name());
    }
}
