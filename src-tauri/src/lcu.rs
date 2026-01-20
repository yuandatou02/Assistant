use crate::shaco::rest::RestClient;
use crate::shaco::utils::get_client_info;
use log::{error, info};
use once_cell::sync::OnceCell;
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
