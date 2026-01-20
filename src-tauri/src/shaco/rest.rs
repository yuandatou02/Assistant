use crate::shaco::utils::build_request_client;

/// REST客户端结构体，用于处理HTTP请求
///
/// # Fields
/// * `port` - 服务器端口号
/// * `request_client` - reqwest客户端实例
pub struct RestClient {
    port: String,
    request_client: reqwest::Client,
}

type Error = Box<dyn std::error::Error>;

impl RestClient {
    /// 创建新的REST客户端实例
    ///
    /// # Arguments
    /// * `auth_token` - 认证令牌，用于构建请求客户端
    /// * `port` - 服务器端口号
    ///
    /// # Returns
    /// * `Result<Self, Error>` - 成功时返回RestClient实例，失败时返回错误
    pub fn new(auth_token: String, port: String) -> Result<Self, Error> {
        let request_client = build_request_client(Some(auth_token));
        Ok(Self {
            port,
            request_client,
        })
    }
}
