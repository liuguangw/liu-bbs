use super::DatabaseError;
use actix_web::{body::BoxBody, http::StatusCode, HttpResponse, ResponseError};
use serde::{ser::SerializeStruct, Serialize, Serializer};
use thiserror::Error;
///api错误
#[derive(Debug, Error)]
pub enum ApiError {
    ///文本描述的错误信息
    #[error("{0}")]
    Common(String),
    ///数据库错误(错误详细信息不会给用户)
    #[error("database error")]
    DatabaseError(#[from] DatabaseError),
}

impl ApiError {
    ///错误代码
    pub fn code(&self) -> u32 {
        match self {
            Self::Common(_) => 5000,
            Self::DatabaseError(_) => 5001,
        }
    }
    ///记录错误信息
    fn log_error(&self) {
        //只记录某些错误
        if let Self::DatabaseError(err) = self {
            println!("[Error#{}] {}", self.code(), err,)
        }
    }
}

impl Serialize for ApiError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("ApiError", 3)?;
        s.serialize_field("code", &self.code())?;
        s.serialize_field("data", &Option::<()>::None)?;
        s.serialize_field("message", &self.to_string())?;
        s.end()
    }
}

impl ResponseError for ApiError {
    fn status_code(&self) -> StatusCode {
        StatusCode::OK
    }
    fn error_response(&self) -> HttpResponse<BoxBody> {
        self.log_error();
        HttpResponse::build(self.status_code()).json(self)
    }
}
