use thiserror::Error;
/// 客户端信息错误枚举
///
/// 用于表示在获取Riot/League客户端信息过程中可能出现的各种错误情况
/// 实现了Error trait以支持错误处理，并实现了Debug和Clone trait以方便调试和使用
#[derive(Error, Debug, Clone)]
pub(crate) enum ClientInfoError {
    /// 进程不可用错误
    /// 当无法找到正在运行的Riot/League客户端进程时返回此错误
    #[error("没有发现Riot/League客户端进程")]
    ProcessNotAvailable,

    /// 端口未找到错误
    /// 当能够找到客户端进程但无法从其启动参数中解析出API端口信息时返回此错误
    #[error("无法从进程参数中解析API端口 ")]
    PortNotFound,

    /// 认证令牌未找到错误
    /// 当能够找到客户端进程但无法从其启动参数中解析出API认证令牌时返回此错误
    #[error("无法从进程参数中解析API认证令牌")]
    AuthTokenNotFound,
}

/// LCU WebSocket错误类型枚举
///
/// 定义了与LCU (League Client Update) WebSocket连接相关的各种错误情况
/// 实现了Debug、Clone和Error trait，便于错误处理和传播
#[derive(Debug, Clone, Error)]
pub enum LcuWebsocketError {
    /// LCU API不可用错误
    ///
    /// 当LCU API服务无法访问时返回此错误，包含具体的错误描述信息
    #[error("LCU API not available: {0}")]
    LcuNotAvailable(String),

    /// 认证错误
    ///
    /// 当WebSocket连接认证失败时返回此错误
    #[error("Authentication error")]
    AuthError,

    /// 消息发送错误
    ///
    /// 当向WebSocket连接发送消息失败时返回此错误
    #[error("Error sending message")]
    SendError,

    /// 连接断开错误
    ///
    /// 当WebSocket连接意外断开时返回此错误，包含断开原因的描述信息
    #[error("Websocket disconnected: {0}")]
    Disconnected(String),
}
