use crate::lcu::lcu_event::LcuSubscriptionType;
use crate::shaco::ws;
use futures_util::StreamExt;
use log::info;
use tauri::{AppHandle, Emitter, EventTarget};

/// 监听LCU WebSocket客户端事件并转发到应用前端
///
/// 该函数连接到LCU WebSocket服务，订阅游戏流程阶段变化事件，
/// 并将接收到的事件数据发送到应用的background标签目标
///
/// # 参数
/// * `app` - 应用句柄，用于向前端发送事件数据
///
/// # 返回值
/// 无返回值，函数持续运行直到连接断开或发生错误
pub async fn listen_client(app: AppHandle) {
    // 连接到LCU WebSocket客户端
    let mut client = ws::LcuWebSocketClient::connect()
        .await
        .expect("获取Lcu WebSocket 失败");

    // 订阅游戏流程阶段变化事件
    client
        .subscribe(LcuSubscriptionType::JsonApiEvent(
            "/lol-gameflow/v1/gameflow-phase".to_string(),
        ))
        .await
        .expect("订阅失败");

    // 持续监听并处理WebSocket事件
    while let Some(event) = client.next().await {
        info!("Event: {:?}", event);
        app.emit_to(
            EventTarget::labeled("background"),
            "client_status",
            event.data,
        )
        .unwrap();
    }
}
