use crate::shaco::rest::RestClient;
use log::info;
use once_cell::sync::OnceCell;

// 定义全局的REST客户端
static REST_CLIENT: OnceCell<RestClient> = OnceCell::new();
// 获取全局REST客户端
fn get_client() -> Result<&'static RestClient, String> {
    REST_CLIENT
        .get()
        .ok_or_else(|| "REST_CLIENT没有初始化!".to_string())
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
