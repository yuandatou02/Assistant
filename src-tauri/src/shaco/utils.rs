use crate::shaco::error::ClientInfoError;
use base64::{Engine, engine::general_purpose};
use reqwest::{Certificate, ClientBuilder, header};
use std::time::Duration;
use sysinfo::{ProcessesToUpdate, System};

#[cfg(target_os = "windows")]
const TARGET_PROCESS: &str = "LeagueClientUx.exe";

/// 获取客户端信息，包括认证令牌和端口号
///
/// 该函数通过系统进程信息查找目标进程，并从其启动参数中提取认证令牌和端口号
///
/// # Returns
/// * `Ok((String, String))` - 成功时返回元组，第一个元素是编码后的认证令牌，第二个元素是端口号
/// * `Err(ClientInfoError)` - 失败时返回相应的错误类型
///
/// # Errors
/// 可能返回以下错误：
/// * `ClientInfoError::ProcessNotAvailable` - 目标进程不可用
/// * `ClientInfoError::PortNotFound` - 未找到端口参数
/// * `ClientInfoError::AuthTokenNotFound` - 未找到认证令牌参数
pub(crate) fn get_client_info() -> Result<(String, String), ClientInfoError> {
    // 初始化系统信息并刷新所有进程数据
    let mut sys_info = System::new_all();
    sys_info.refresh_processes(ProcessesToUpdate::All, true);

    // 查找目标进程的命令行参数
    let args = sys_info
        .processes()
        .values()
        .find(|p| p.name() == TARGET_PROCESS)
        .map(|p| p.cmd())
        .ok_or(ClientInfoError::ProcessNotAvailable)?;

    // 从参数中提取端口号
    let port = args
        .iter()
        .find(|arg| arg.to_string_lossy().starts_with("--app-port="))
        .map(|arg| {
            arg.to_string_lossy()
                .strip_prefix("--app-port=")
                .expect("获取端口失败")
                .to_string()
        })
        .ok_or(ClientInfoError::PortNotFound)?;

    // 从参数中提取认证令牌
    let auth_token = args
        .iter()
        .find(|arg| arg.to_string_lossy().starts_with("--remoting-auth-token="))
        .map(|arg| {
            arg.to_string_lossy()
                .strip_prefix("--remoting-auth-token=")
                .expect("获取认证令牌失败")
                .to_string()
        })
        .ok_or(ClientInfoError::AuthTokenNotFound)?;

    // 对认证令牌进行Base64编码并添加riot前缀，然后返回结果
    Ok((
        general_purpose::STANDARD.encode(format!("riot:{}", auth_token)),
        port,
    ))
}

/// 构建HTTP请求客户端
///
/// 该函数用于创建一个配置好的reqwest客户端，用于发送HTTP请求。
/// 客户端会配置自定义证书、认证头信息以及请求超时时间。
///
/// # 参数
///
/// * `auth_token` - 可选的认证令牌字符串。如果提供，将作为Basic认证头添加到请求中
///
/// # 返回值
///
/// 返回配置好的reqwest::Client实例
pub(crate) fn build_request_client(auth_token: Option<String>) -> reqwest::Client {
    // 加载自定义根证书用于SSL验证
    let cert = Certificate::from_pem(include_bytes!("./riotgames.pem")).expect("加载证书失败!");

    // 创建并配置请求头，包括可选的认证信息
    let mut headers = header::HeaderMap::new();
    if let Some(token) = auth_token {
        let auth_header = header::HeaderValue::from_str(format!("Basic {}", token).as_str())
            .expect("创建请求头失败!");
        headers.insert(header::AUTHORIZATION, auth_header);
    }

    // 构建客户端实例，配置证书、默认请求头和超时时间
    ClientBuilder::new()
        .add_root_certificate(cert)
        .default_headers(headers)
        .timeout(Duration::from_secs(3))
        .build()
        .expect("创建请求客户端失败!")
}
