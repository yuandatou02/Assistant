use reqwest::{Certificate, ClientBuilder, header};
use std::time::Duration;

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
    let cert = Certificate::from_pem(include_bytes!("riotgames.pem")).expect("加载证书失败!");

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
