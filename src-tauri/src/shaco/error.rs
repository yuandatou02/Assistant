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
