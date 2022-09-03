use super::api_error::ApiError;
use rocket::response::Responder;
use rocket::serde::json::Json;
use rocket::serde::ser::{Serialize, SerializeStruct, Serializer};
use rocket::{response, Request};
///api响应数据
pub struct ResponseData<T: Serialize>(T);
impl<T: Serialize> Serialize for ResponseData<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("ResponseData", 3)?;
        s.serialize_field("code", &0u32)?;
        s.serialize_field("data", &self.0)?;
        s.serialize_field("message", "")?;
        s.end()
    }
}
impl<'r, T: Serialize> Responder<'r, 'static> for ResponseData<T> {
    fn respond_to(self, req: &'r Request<'_>) -> response::Result<'static> {
        Json(&self).respond_to(req)
    }
}
impl<T: Serialize> From<T> for ResponseData<T> {
    fn from(data: T) -> Self {
        Self(data)
    }
}
///api响应
pub type ResponseResult<T> = Result<ResponseData<T>, ApiError>;
