use thiserror::Error;
/// 数据迁移错误
#[derive(Debug, Error)]
pub enum MigrationError {
    ///数据库错误
    #[error("{0:?}")]
    DatabaseError(#[from] mongodb::error::Error),
    ///文本描述的错误信息
    #[error("{0}")]
    Common(String),
}
