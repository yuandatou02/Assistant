use ahash::AHashMap;
use once_cell::sync::Lazy;

use crate::lcu::types::{champion_mastery::ChampionInfo, rank_info::RankedEntry};

/// 英文 tier → 中文
fn english_to_chinese(tier: &str) -> &'static str {
    match tier {
        "CHALLENGER" => "王者",
        "GRANDMASTER" => "宗师",
        "MASTER" => "大师",
        "DIAMOND" => "钻石",
        "EMERALD" => "翡翠",
        "PLATINUM" => "铂金",
        "GOLD" => "黄金",
        "SILVER" => "白银",
        "BRONZE" => "青铜",
        "IRON" => "黑铁",
        _ => "未定级",
    }
}

/// 把 division 罗马数字转成中文习惯
fn deal_division(d: &str) -> &'static str {
    match d {
        "I" => "Ⅰ",
        "II" => "Ⅱ",
        "III" => "Ⅲ",
        "IV" => "Ⅳ",
        _ => "",
    }
}

/// 生成一条排名字符串
pub fn generate_rank_string(entry: Option<&RankedEntry>) -> String {
    let Some(e) = entry else {
        return "未定级".to_string();
    };
    if e.tier.is_empty() {
        return "未定级".to_string();
    }
    format!(
        "{}{}{}",
        english_to_chinese(&e.tier),
        deal_division(&e.division),
        e.league_points
    )
}

pub static CHAMP_DICT: Lazy<AHashMap<String, ChampionInfo>> = Lazy::new(|| {
    serde_json::from_str(include_str!("../resource/champ_dict.json")).expect("解析 JSON 失败")
});

pub fn get_champion(id: String) -> Option<&'static ChampionInfo> {
    CHAMP_DICT.get(&id)
}
