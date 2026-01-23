/// 整个玩家排位信息根结构
#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct RankedInfo {
    /// 已获得的奖励皮肤ID列表
    pub earned_regalia_reward_ids: Vec<String>,

    /// 上赛季结束时段位
    pub highest_previous_season_end_division: String,
    /// 上赛季结束时等级
    pub highest_previous_season_end_tier: String,

    /// 历史最高排位条目（可能为灵活或单双排）
    pub highest_ranked_entry: Option<RankedEntry>,
    /// 历史最高单双排排位条目
    pub highest_ranked_entry_sr: Option<RankedEntry>,

    /// 按队列类型（RANKED_SOLO_5x5 等）区分的详细条目
    pub queue_map: std::collections::HashMap<String, RankedEntry>,

    /// 所有队列的条目数组（与 queue_map 信息重复，保留兼容）
    pub queues: Vec<RankedEntry>,

    /// 排位装饰等级（0 为无）
    pub ranked_regalia_level: u32,

    /// 各赛季起止时间信息
    pub seasons: std::collections::HashMap<String, SeasonInfo>,

    /// 分段进度（key 为 split 名，value 为进度值）
    pub splits_progress: std::collections::HashMap<String, u32>,
}

/// 单个队列的排位条目
#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct RankedEntry {
    /// 当前段位（如 "I", "II"）
    pub division: String,
    /// 是否为临时段位（定位赛阶段）
    pub is_provisional: bool,
    /// 当前胜点（LP）
    pub league_points: u32,
    /// 败场数
    pub losses: u32,
    /// 晋级赛进度字符串（如 "WWL"），非晋级赛为 ""
    pub mini_series_progress: String,
    /// 上赛季结束时段位
    pub previous_season_end_division: String,
    /// 上赛季结束时等级
    pub previous_season_end_tier: String,

    /// 进入临时段位所需总场次
    pub provisional_game_threshold: u32,
    /// 剩余定位赛场次
    pub provisional_games_remaining: u32,
    /// 队列类型（RANKED_SOLO_5x5, RANKED_FLEX_SR 等）
    pub queue_type: String,
    /// 用于 Rated 模式的评分值
    pub rated_rating: u32,
    /// Rated 模式等级
    pub rated_tier: String,
    /// 当前等级（如 "GOLD", "PLATINUM"）
    pub tier: String,

    /// 警告信息
    pub warnings: Option<Warnings>,
    /// 胜场数
    pub wins: u32,
}

/// 段位衰减相关警告
#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct Warnings {
    /// 距离衰减还剩天数
    pub days_until_decay: u32,
    /// 降级警告级别（0 为无）
    pub demotion_warning: u32,
    /// 是否显示衰减警告
    pub display_decay_warning: bool,
    /// 距离不活跃状态变化还剩秒数
    pub time_until_inactivity_status_changes: i64,
}

/// 赛季起止时间
#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct SeasonInfo {
    /// 当前赛季结束时间（Unix 毫秒）
    pub current_season_end: i64,
    /// 当前赛季 ID
    pub current_season_id: u32,
    /// 下赛季开始时间（Unix 毫秒）
    pub next_season_start: i64,
}
