use serde::Deserialize;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use tokio::fs;

/// 服务配置
#[derive(Deserialize)]
pub struct ServerConfig {
    ///ip
    pub host: String,
    ///端口
    pub port: u16,
}

///应用配置
#[derive(Deserialize)]
pub struct AppConfig {
    /// 服务配置
    pub server: ServerConfig,
}

///配置加载错误
#[derive(Debug)]
pub enum AppConfigError {
    ///配置文件读取错误
    IoErr(std::io::Error),
    ///配置文件解析错误
    ParseErr(toml::de::Error),
}

impl From<std::io::Error> for AppConfigError {
    fn from(e: std::io::Error) -> Self {
        Self::IoErr(e)
    }
}

impl From<toml::de::Error> for AppConfigError {
    fn from(e: toml::de::Error) -> Self {
        Self::ParseErr(e)
    }
}
impl Display for AppConfigError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match &self {
            Self::IoErr(e) => <std::io::Error as Display>::fmt(e, f),
            Self::ParseErr(e) => <toml::de::Error as Display>::fmt(e, f),
        }
    }
}

impl std::error::Error for AppConfigError {}

impl AppConfig {
    /// 加载配置
    pub async fn load(file_path: &str) -> Result<Self, AppConfigError> {
        let contents = fs::read_to_string(file_path).await?;
        let s: Self = toml::from_str(&contents)?;
        Ok(s)
    }
}
