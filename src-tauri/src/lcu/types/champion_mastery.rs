use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct ChampionMastery {
    pub champion_id: i64,
    pub champion_level: i64,
    pub champion_points: i64,
    pub champion_points_since_last_level: i64,
    pub champion_points_until_next_level: i64,
    pub champion_season_milestone: i64,
    pub highest_grade: String,
    pub last_play_time: i64,
    pub mark_required_for_next_level: i64,
    pub milestone_grades: Vec<String>,
    pub next_season_milestone: NextSeasonMilestone,
    pub puuid: String,
    pub tokens_earned: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct NextSeasonMilestone {
    pub bonus: bool,
    pub require_grade_counts: RequireGradeCounts,
    pub reward_config: RewardConfig,
    pub reward_marks: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct RequireGradeCounts {
    #[serde(rename = "A-", default)]
    pub a_minus: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct RewardConfig {
    pub maximum_reward: i64,
    pub reward_value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChampionInfo {
    pub label: String,
    pub alias: String,
    pub title: String,
    #[serde(default)]
    pub roles: Vec<String>,
}
