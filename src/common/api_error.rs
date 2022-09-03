use rocket::response::Responder;
use rocket::serde::json::Json;
use rocket::serde::ser::{Serialize, SerializeStruct, Serializer};
use rocket::{response, Request};
///api错误
pub enum ApiError {
    ///文本描述的错误信息
    Common(String),
    ///数据库错误
    DatabaseError(mongodb::error::Error),
}

impl ApiError {
    pub fn code(&self) -> u32 {
        match &self {
            ApiError::Common(_) => 5000,
            ApiError::DatabaseError(_) => 5001,
        }
    }
    pub fn message(&self) -> String {
        match &self {
            ApiError::Common(e) => e.to_string(),
            ApiError::DatabaseError(e) => format!("{:?}", e),
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
        s.serialize_field("message", &self.message())?;
        s.end()
    }
}

impl<'r> Responder<'r, 'static> for ApiError {
    fn respond_to(self, req: &'r Request<'_>) -> response::Result<'static> {
        Json(&self).respond_to(req)
    }
}

impl From<mongodb::error::Error> for ApiError {
    fn from(e: mongodb::error::Error) -> Self {
        Self::DatabaseError(e)
    }
}
