pub mod lcu_event;
mod listener;
mod types;
mod utils;

use crate::lcu::listener::listen_client;
use crate::lcu::types::champion_mastery::ChampionMastery;
use crate::lcu::types::rank_info::{HonorProfile, RankedInfo};
use crate::lcu::types::summoner_info::{LcuSummonerInfo, SummonerInfo};
use crate::lcu::utils::{generate_rank_string, get_champion};
use crate::shaco::rest::RestClient;
use crate::shaco::utils::get_client_info;
use log::{error, info};
use once_cell::sync::OnceCell;
use serde_json::Value;
use std::time::{Duration, Instant};
use tauri::{AppHandle, Emitter};

// 定义全局的REST客户端
static REST_CLIENT: OnceCell<RestClient> = OnceCell::new();
// 获取全局REST客户端
fn get_client() -> Result<&'static RestClient, String> {
    REST_CLIENT
        .get()
        .ok_or_else(|| "REST_CLIENT没有初始化!".to_string())
}
#[tauri::command]
pub fn start_listener(app: AppHandle) {
    tokio::spawn(async move {
        listen_client(app).await;
    });
}

/// 获取 mastery 英雄列表
/// 
/// # 参数
/// * `endpoint` - 请求的端点地址
/// 
/// # 返回值
/// * `Result<Vec<Vec<String>>, Value>` - 成功时返回包含英雄信息的二维字符串向量，失败时返回 Value 类型错误
#[tauri::command]
pub async fn get_mastery_champ_list(endpoint: &str) -> Result<Vec<Vec<String>>, Value> {
    info!("请求uri为:{}", endpoint);
    let client = get_client().map_err(|_| Value::Null)?;
    let value = client.get(endpoint).await.map_err(|_| Value::Null)?;
    // info!("get_mastery_champ_list获取的value为:{:#?}", value);

    // 反序列化响应数据为 ChampionMastery 结构体向量
    let masteries: Vec<ChampionMastery> = serde_json::from_value(value).map_err(|e| {
        error!("反序列化失败:{}", e);
        Value::Null
    })?;

    // 处理英雄数据，提取前20个英雄并格式化为所需的数据结构
    let result: Vec<Vec<String>> = masteries
        .into_iter()
        .take(20)
        .filter_map(|item| {
            let champ_id = item.champion_id.to_string();
            let champ_info = get_champion(champ_id)?;
            Some(vec![
                format!(
                    "https://game.gtimg.cn/images/lol/act/img/champion/{}.png",
                    champ_info.alias
                ),
                format!("{}•{}", champ_info.label, champ_info.title),
                format!(
                    "英雄等级 {} / 熟练度 {}",
                    item.champion_level, item.champion_points
                ),
            ])
        })
        .collect();
    // info!("获取的英雄列表为:{:#?}", result);
    Ok(result)
}

/// 获取召唤师荣誉信息
///
/// 该函数通过Tauri命令接口异步获取当前召唤师的荣誉等级和里程碑信息，
/// 并将其格式化为字符串返回。
///
/// # Returns
/// * `Result<String, Value>` - 成功时返回格式化的荣誉信息字符串，失败时返回Value::Null
///   格式为："荣誉等级{level} 里程{checkpoint}"
#[tauri::command]
pub async fn get_summoner_honor() -> Result<String, Value> {
    // 创建HTTP客户端并处理错误
    let client = get_client().map_err(|_| Value::Null)?;
    // 发起GET请求获取荣誉配置文件数据
    let value = client
        .get("/lol-honor-v2/v1/profile")
        .await
        .map_err(|_| Value::Null)?;
    // 将响应数据反序列化为HonorProfile结构体
    let result: HonorProfile = serde_json::from_value(value).map_err(|_| Value::Null)?;
    // 格式化荣誉等级和里程碑信息并返回
    Ok(format!(
        "荣誉等级{} 里程{}",
        result.honor_level, result.checkpoint
    ))
}

/// 获取用户在不同游戏模式下的排位积分信息
///
/// 该函数通过API端点获取用户的排位信息，并将其转换为包含三种游戏模式（单双排、灵活组排、云顶之弈）
/// 排位等级的字符串数组。如果用户没有排位数据，则返回"未定级"。
///
/// # 参数
/// * `endpoint` - API端点URL字符串引用，用于获取排位信息
///
/// # 返回值
/// * `Result<[String; 3], Value>` - 成功时返回包含三个排位等级字符串的数组，
///   分别对应单双排、灵活组排、云顶之弈模式；失败时返回Value::Null
///
/// # 错误处理
/// 当网络请求失败、反序列化失败或客户端创建失败时，返回Value::Null
#[tauri::command]
pub async fn get_rank_point(endpoint: &str) -> Result<[String; 3], Value> {
    info!("请求uri为:{}", endpoint);
    let client = get_client().map_err(|_| Value::Null)?;
    let value = client.get(endpoint).await.map_err(|_| Value::Null)?;
    let value: RankedInfo = serde_json::from_value(value).map_err(|e| {
        error!("反序列化 RankedInfo 失败: {:?}", e);
        Value::Null
    })?;
    let queues = value.queues;
    // 如果没有排位数据，返回 ["未定级"; 3] 的 JSON 数组
    if queues.is_empty() {
        Ok([
            "未定级".to_string(),
            "未定级".to_string(),
            "未定级".to_string(),
        ])
    } else {
        // 查找三种不同游戏模式的排位信息
        let rank_solo = queues.iter().find(|q| q.queue_type == "RANKED_SOLO_5x5");
        let rank_flex = queues.iter().find(|q| q.queue_type == "RANKED_FLEX_SR");
        let rank_tft = queues.iter().find(|q| q.queue_type == "RANKED_TFT");
        // 获取各模式的排名信息
        let ranked_solo = generate_rank_string(rank_solo);
        let ranked_flex_sr = generate_rank_string(rank_flex);
        let ranked_tft = generate_rank_string(rank_tft);
        info!(
            "获取到的信息为:ranked_solo:{} ranked_flex_sr:{} ranked_tft:{}",
            ranked_solo, ranked_flex_sr, ranked_tft
        );
        Ok([ranked_solo, ranked_flex_sr, ranked_tft])
    }
}

/// 获取召唤师信息
///
/// 该函数通过Tauri命令接口，向指定端点发送HTTP请求获取召唤师数据，
/// 并将LCU格式的数据转换为应用所需的SummonerInfo格式。
///
/// # 参数
/// * `endpoint` - 请求的API端点URL字符串引用
///
/// # 返回值
/// * `Result<SummonerInfo, Value>` - 成功时返回SummonerInfo结构体，失败时返回Value::Null
///
/// # 错误处理
/// 当客户端创建失败、网络请求失败或JSON解析失败时，返回Value::Null
#[tauri::command]
pub async fn get_summoner_info(endpoint: &str) -> Result<SummonerInfo, Value> {
    info!("请求uri为:{}", endpoint);

    // 创建HTTP客户端
    let client = get_client().map_err(|_| Value::Null)?;

    // 发送GET请求获取原始数据
    let value = client.get(endpoint).await.map_err(|_| Value::Null)?;

    // 将JSON值解析为LCU召唤师信息结构体
    let info: LcuSummonerInfo = serde_json::from_value(value).map_err(|_| Value::Null)?;

    info!("获取到的信息为:{:?}", info);

    // 将LCU格式的召唤师信息转换为应用所需的格式
    Ok(SummonerInfo{
        privacy:info.privacy,
        puuid:info.puuid,
        tag_line: info.tag_line,
        name:if info.game_name.is_empty() {
            info.display_name
        } else {
            info.game_name
        },
        current_id: info.summoner_id,
        lv: format!("Lv {}",info.summoner_level).to_string(),

        // 计算经验值百分比
        xp: ((info.xp_since_last_level as f64 / info.xp_until_next_level as f64) * 100.0) as i64,

        // 构建头像图片URL
        img_url: format!("https://wegame.gtimg.com/g.26-r.c2d3c/helper/lol/assis/images/resources/usericon/{}.png",info.profile_icon_id).to_string()
    })
}

/// 启动客户端并检查客户端信息获取是否成功
///
/// 该函数通过调用get_client_info()来验证客户端是否能够正常启动
/// 并返回操作结果状态
///
/// # Returns
///
/// * `bool` - 如果客户端信息获取成功则返回true，否则返回false
#[tauri::command]
pub fn start_client() -> bool {
    info!("客户端已成功启动!");
    get_client_info().is_ok()
}

/// 获取游戏路径
///
/// 该函数通过Tauri命令接口异步获取游戏安装路径，并将其转换为特定格式
///
/// # Returns
/// * `Result<String, Value>` - 成功时返回游戏路径字符串，失败时返回Value::Null
///
/// # Errors
/// 当获取客户端失败或HTTP请求失败时返回错误
#[tauri::command]
pub async fn get_game_path() -> Result<String, Value> {
    // 获取HTTP客户端实例
    let client = get_client()?;

    // 发起HTTP GET请求获取安装目录信息
    let game_path = client
        .get("/data-store/v1/install-dir")
        .await
        .map_err(|_| Value::Null)?
        .as_str()
        .expect("路径转换失败!")
        .replace("LeagueClient", r"TCLS\client.exe");

    // 记录获取到的游戏路径信息
    info!("获取到的游戏路径为:{}", game_path);
    Ok(game_path)
}

/// 监听客户端启动状态并初始化REST客户端
///
/// 该函数会在后台持续检查客户端信息，直到获取到有效的认证令牌和端口信息，
/// 或者超过指定的超时时间。成功获取信息后会初始化全局REST客户端并发送状态通知。
///
/// # 参数
/// * `app` - Tauri应用句柄，用于发送事件通知
///
/// # 返回值
/// 无返回值，函数在后台异步执行
#[tauri::command]
pub fn listen_client_start(app: AppHandle) {
    tokio::spawn(async move {
        let start_time = Instant::now();
        let timeout = Duration::from_secs(180);
        loop {
            if let Ok(value) = get_client_info() {
                info!("找到客户端信息:auth_token:{},port:{}", value.0, value.1);
                let _ = REST_CLIENT
                    .set(RestClient::new(value.0, value.1).unwrap())
                    .map_err(|_| "REST_CLIENT is already initialized".to_string());
                app.emit_to("background", "client_status", "ClientStarted")
                    .expect("sent background error");
                break; // 找到客户端信息后退出循环
            }
            // 超过指定的超时时间则退出
            if start_time.elapsed() > timeout {
                error!("客户端启动超时，未能获取信息。");
                break;
            }

            // 每隔一段时间重新检查
            tokio::time::sleep(Duration::from_secs(3)).await;
        }
    });
}

/// 启动游戏命令函数
///
/// 该函数用于启动指定路径的游戏程序
///
/// # 参数
/// * [path](file://E:\code\rust\assistant\src\main\views\home\startGame.vue#L63-L63) - 游戏可执行文件的路径字符串引用
///
/// # 返回值
/// 无返回值
///
/// # 注意
/// 此函数使用 Tauri 命令宏标记，只能在 Tauri 应用中调用
#[tauri::command]
pub fn start_game(path: &str) {
    // 记录游戏启动日志
    info!("启动的游戏路径为:{}", path);
    // 创建并启动游戏进程
    std::process::Command::new(path)
        .spawn()
        .expect("无法启动游戏");
}
