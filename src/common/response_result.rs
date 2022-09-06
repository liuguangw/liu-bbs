use super::ApiError;
use actix_web::{body::EitherBody, web::Json, HttpRequest, HttpResponse, Responder};
use serde::{ser::SerializeStruct, Serialize, Serializer};

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
impl<T: Serialize> Responder for ResponseData<T> {
    type Body = EitherBody<String>;
    fn respond_to(self, req: &HttpRequest) -> HttpResponse<Self::Body> {
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
