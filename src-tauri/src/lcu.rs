pub mod lcu_event;
mod listener;

use crate::lcu::listener::listen_client;
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
