use std::fmt::Display;

///数据库错误
#[derive(Debug)]
pub struct DatabaseError(mongodb::error::Error);

impl Display for DatabaseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", &self.0)
    }
}

impl std::error::Error for DatabaseError {}

impl From<mongodb::error::Error> for DatabaseError {
    fn from(e: mongodb::error::Error) -> Self {
        Self(e)
    }
}
///从数据库获取的数据
pub type DatabaseResult<T> = Result<T, DatabaseError>;
