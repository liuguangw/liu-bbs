use serde::Deserialize;
/// 服务配置
#[derive(Deserialize)]
#[serde(default)]
pub struct ServerConfig {
    ///ip
    pub host: String,
    ///端口
    pub port: u16,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            host: String::from("127.0.0.1"),
            port: 8000,
        }
    }
}
