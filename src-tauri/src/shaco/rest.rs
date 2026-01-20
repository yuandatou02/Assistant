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

    /// 发送GET请求到本地指定端口的API端点
    ///
    /// # 参数
    /// * `endpoint` - API端点路径，将与基础URL组合成完整的请求地址
    ///
    /// # 返回值
    /// * `Result<serde_json::Value, reqwest::Error>` - 成功时返回解析的JSON值，失败时返回请求错误
    ///   如果JSON解析失败，将返回`serde_json::Value::Null`
    pub async fn get(&self, endpoint: &str) -> Result<serde_json::Value, reqwest::Error> {
        // 构建完整的请求URL并发送GET请求
        self.request_client
            .get(format!("https://127.0.0.1:{}{}", self.port, endpoint))
            .send()
            .await?
            .error_for_status()?
            .json()
            .await
            .or_else(|_| Ok(serde_json::Value::Null))
    }
}
