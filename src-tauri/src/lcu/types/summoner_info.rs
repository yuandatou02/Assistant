use serde::{Deserialize, Serialize};

/// 召唤师信息结构体，包含召唤师的基本信息、隐私设置、等级进度等数据
///
/// 该结构体用于存储和传输召唤师相关的详细信息，包括账户标识、显示名称、
/// 游戏进度、隐私状态、头像ID、重-roll点数等属性
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LcuSummonerInfo {
    /// 账户唯一标识符
    pub account_id: i64,
    /// 显示名称（可能与游戏名称不同）
    pub display_name: String,
    /// 游戏内使用的名称
    pub game_name: String,
    /// 内部系统使用的名称
    pub internal_name: String,
    /// 名称更改标志，指示是否最近更改过名称
    pub name_change_flag: bool,
    /// 达到下一级所需经验的完成百分比
    pub percent_complete_for_next_level: i32,
    /// 隐私设置（公开或私有）
    pub privacy: Privacy,
    /// 个人资料头像图标ID
    pub profile_icon_id: i32,
    /// PUUID（Player Universal Unique Identifier）- 玩家通用唯一标识符
    pub puuid: String,
    /// 重-roll点数相关信息
    pub reroll_points: RerollPoints,
    /// 召唤师唯一标识符
    pub summoner_id: i64,
    /// 当前召唤师等级
    pub summoner_level: i64,
    /// 标签行（通常用于区分同名玩家）
    pub tag_line: String,
    /// 是否未命名状态
    pub unnamed: bool,
    /// 自上次升级以来获得的经验值
    pub xp_since_last_level: i64,
    /// 升级到下一级还需要的经验值
    pub xp_until_next_level: i64,
}

/// 隐私设置枚举，定义了用户信息的可见性级别
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum Privacy {
    /// 公开 - 信息对所有用户可见
    Public,
    /// 私有 - 信息仅对自己可见
    Private,
}

/// 重-roll点数结构体，管理用户在某些功能中重新选择的机会点数
///
/// 该结构体用于跟踪和管理用户进行重-roll操作时的点数状态，
/// 包括当前可用点数、最大可roll次数、已使用次数等信息
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RerollPoints {
    /// 当前拥有的重-roll点数
    pub current_points: i32,
    /// 最大允许的roll次数
    pub max_rolls: i32,
    /// 已经进行的roll次数
    pub number_of_rolls: i32,
    /// 每次roll操作消耗的点数
    pub points_cost_to_roll: i32,
    /// 执行一次重-roll所需的点数
    pub points_to_reroll: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SummonerInfo {
    pub name: String,
    pub privacy: Privacy,
    pub img_url: String,
    pub lv: String,
    pub xp: i64,
    pub puuid: String,
    pub current_id: i64,
    pub tag_line: String,
}
