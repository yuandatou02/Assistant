use crate::lcu::lcu_event::{LcuEvent, LcuSubscriptionType};
use crate::shaco::error::LcuWebsocketError;
use crate::shaco::utils::get_client_info;
use futures_util::{SinkExt, Stream, StreamExt};
use native_tls::{Certificate, TlsConnector};
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::net::TcpStream;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::tungstenite::client::IntoClientRequest;
use tokio_tungstenite::tungstenite::http::HeaderValue;
use tokio_tungstenite::tungstenite::http::header::AUTHORIZATION;
use tokio_tungstenite::{Connector, MaybeTlsStream, WebSocketStream, tungstenite};

#[derive(Debug)]
pub struct LcuWebSocketClient(WebSocketStream<MaybeTlsStream<TcpStream>>);

impl LcuWebSocketClient {
    /// 连接到LCU WebSocket服务
    ///
    /// 该方法会获取客户端认证信息，建立TLS连接，并使用基本认证进行WebSocket握手
    ///
    /// # Returns
    /// * `Ok(LcuWebSocketClient)` - 成功连接时返回WebSocket客户端实例
    /// * `Err(LcuWebsocketError)` - 连接失败时返回相应的错误
    ///
    /// # Errors
    /// 可能返回以下错误类型：
    /// * `LcuWebsocketError::LcuNotAvailable` - LCU客户端不可用或无法获取连接信息
    /// * `LcuWebsocketError::AuthError` - 认证失败
    /// * `LcuWebsocketError::Disconnected` - WebSocket连接失败
    pub async fn connect() -> Result<Self, LcuWebsocketError> {
        // 获取LCU客户端的认证令牌和端口号
        let (auth_token, port) =
            get_client_info().map_err(|e| LcuWebsocketError::LcuNotAvailable(e.to_string()))?;

        // 加载Riot Games的根证书用于TLS验证
        let cert = Certificate::from_pem(include_bytes!("./riotgames.pem")).expect("证书错误");

        // 构建TLS连接器，添加自定义根证书
        let tls = TlsConnector::builder()
            .add_root_certificate(cert)
            .build()
            .unwrap();
        let connector = Connector::NativeTls(tls);

        // 构建WebSocket连接URL并设置认证头
        let mut url = format!("wss://127.0.0.1:{}", port)
            .into_client_request()
            .map_err(|_| LcuWebsocketError::AuthError)?;
        url.headers_mut().insert(
            AUTHORIZATION,
            HeaderValue::from_str(format!("Basic {}", auth_token).as_str())
                .map_err(|_| LcuWebsocketError::AuthError)?,
        );

        // 建立TLS加密的WebSocket连接
        let (ws_stream, _) =
            tokio_tungstenite::connect_async_tls_with_config(url, None, false, Some(connector))
                .await
                .map_err(|e| LcuWebsocketError::Disconnected(e.to_string()))?;
        Ok(Self(ws_stream))
    }

    /// 订阅LCU WebSocket事件
    ///
    /// 该函数向LCU WebSocket服务器发送订阅请求，指定要监听的事件类型。
    /// 订阅消息格式遵循LCU WebSocket协议，使用"[5, "{subscription}"]"格式。
    ///
    /// # 参数
    /// * `subscription` - 要订阅的LCU事件类型，实现Display trait以转换为字符串
    ///
    /// # 返回值
    /// * `Ok(())` - 订阅成功
    /// * `Err(LcuWebsocketError)` - 订阅失败，可能的原因包括连接已关闭或发送错误
    ///
    /// # 错误处理
    /// - 当WebSocket连接已关闭时，返回`LcuWebsocketError::Disconnected`
    /// - 其他发送错误返回`LcuWebsocketError::SendError`
    pub async fn subscribe(
        &mut self,
        subscription: LcuSubscriptionType,
    ) -> Result<(), LcuWebsocketError> {
        // 发送订阅消息到WebSocket连接，并处理可能的错误
        self.0
            .send(Message::text(format!("[5, \"{subscription}\"]")))
            .await
            .map_err(|e| match e {
                tungstenite::Error::ConnectionClosed | tungstenite::Error::AlreadyClosed => {
                    LcuWebsocketError::Disconnected(e.to_string())
                }
                _ => LcuWebsocketError::SendError,
            })
    }
}

impl Stream for LcuWebSocketClient {
    type Item = LcuEvent;

    /// 异步轮询WebSocket消息流
    ///
    /// 该方法持续监听WebSocket连接中的文本消息，将其解析为LcuEvent对象
    /// 当收到关闭消息、错误或连接断开时返回None
    ///
    /// # Parameters
    /// * `self` - LcuWebSocketClient的可变引用
    /// * `cx` - Waker上下文，用于异步任务调度
    ///
    /// # Returns
    /// * `Poll::Ready(Some(LcuEvent))` - 收到有效事件消息时返回
    /// * `Poll::Ready(None)` - 连接关闭或出错时返回
    /// * `Poll::Pending` - 暂无数据可读时返回
    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        loop {
            return match self.0.poll_next_unpin(cx) {
                Poll::Pending => Poll::Pending,
                Poll::Ready(Some(Ok(Message::Text(text)))) => {
                    let Ok(event) = serde_json::from_str::<LcuEvent>(&text) else {
                        continue;
                    };
                    Poll::Ready(Some(event))
                }
                Poll::Ready(Some(Ok(Message::Close(_))) | Some(Err(_)) | None) => Poll::Ready(None),
                _ => continue,
            };
        }
    }
}
