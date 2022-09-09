use super::{AppConfigError, DatabaseError};
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
    ServerError(#[from] std::io::Error),
    ///数据库错误
    #[error("{0}")]
    DatabaseError(DatabaseError),
    ///数据迁移错误
    #[error("{0}")]
    MigrationError(#[from] MigrationError),
}

impl From<mongodb::error::Error> for LaunchError {
    fn from(e: mongodb::error::Error) -> Self {
        let db_error = DatabaseError::from(e);
        Self::DatabaseError(db_error)
    }
}
