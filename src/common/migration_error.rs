use thiserror::Error;
/// 服务启动错误
#[derive(Debug, Error)]
pub enum MigrationError {
    ///io错误
    #[error("{0}")]
    IoError(#[from] std::io::Error),
    ///数据库错误
    #[error("{0:?}")]
    DatabaseError(#[from] mongodb::error::Error),
    ///文本描述的错误信息
    #[error("{0}")]
    Common(String),
}
