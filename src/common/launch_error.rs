use super::AppConfigError;
use thiserror::Error;
/// 服务启动错误
#[derive(Debug, Error)]
pub enum LaunchError {
    ///配置加载错误
    #[error("{0}")]
    ConfigError(#[from] AppConfigError),
    ///服务启动错误
    #[error("{0}")]
    RocketError(Box<rocket::Error>),
    ///数据库错误
    #[error("{0:?}")]
    DatabaseError(#[from] mongodb::error::Error),
}

impl From<rocket::Error> for LaunchError {
    fn from(e: rocket::Error) -> Self {
        Self::RocketError(Box::new(e))
    }
}
