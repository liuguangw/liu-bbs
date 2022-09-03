use super::AppConfigError;
use super::DatabaseConfig;
use super::ServerConfig;
use serde::Deserialize;
use tokio::fs;

///应用配置
#[derive(Deserialize, Default)]
#[serde(default)]
pub struct AppConfig {
    /// 服务配置
    pub server: ServerConfig,
    ///数据库配置
    pub database: DatabaseConfig,
}

impl AppConfig {
    /// 加载配置
    pub async fn load(file_path: &str) -> Result<Self, AppConfigError> {
        let contents = fs::read_to_string(file_path)
            .await
            //std::io::Error -> AppConfigError
            .map_err(|err| AppConfigError::IoErr(file_path.to_string(), err))?;
        let s: Self = toml::from_str(&contents)
            //toml::de::Error -> AppConfigError
            .map_err(|err| AppConfigError::ParseErr(file_path.to_string(), err))?;
        Ok(s)
    }
}
