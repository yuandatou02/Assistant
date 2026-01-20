use crate::lcu::lcu_event::LcuSubscriptionType;
use crate::shaco::ws;
use futures_util::StreamExt;
use log::info;
use tauri::{AppHandle, Emitter, EventTarget};

pub async fn listen_client(app: AppHandle) {
    let mut client = ws::LcuWebSocketClient::connect()
        .await
        .expect("获取Lcu WebSocket 失败");
    client
        .subscribe(LcuSubscriptionType::JsonApiEvent(
            "/lol-gameflow/v1/gameflow-phase".to_string(),
        ))
        .await
        .expect("订阅失败");
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
