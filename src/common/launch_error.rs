use super::AppConfigError;
use crate::common::MigrationError;
use thiserror::Error;

/// 服务启动错误
#[derive(Debug, Error)]
pub enum LaunchError {
    ///配置加载错误
    #[error("{0}")]
    ConfigError(#[from] AppConfigError),
    ///服务启动错误
    #[error("{0}")]
    ServerError(#[from] actix_web::Error),
    ///服务启动错误
    #[error("{0}")]
    BindError(#[from] std::io::Error),
    ///数据库错误
    #[error("{0:?}")]
    DatabaseError(#[from] mongodb::error::Error),
    ///数据迁移错误
    #[error("{0}")]
    MigrationError(#[from] MigrationError),
}
