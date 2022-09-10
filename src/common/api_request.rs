use std::{future::Future, ops::Deref, pin::Pin};

use actix_web::{web::Json, FromRequest};
use serde::de::DeserializeOwned;

use super::ApiError;

///检测用户的输入请求
pub trait ApiRequestValidator {
    ///检测用户输入
    fn check_input(&self) -> Result<(), ApiError>;
}

///api请求封装
pub struct ApiRequest<T>
where
    T: DeserializeOwned + ApiRequestValidator,
{
    inner_item: Json<T>,
}
//T不能是引用类型
impl<T: DeserializeOwned + ApiRequestValidator + 'static> FromRequest for ApiRequest<T> {
    type Error = ApiError;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(
        req: &actix_web::HttpRequest,
        payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        let s = Json::<T>::from_request(req, payload);
        Box::pin(async move {
            match s.await {
                //输入验证
                Ok(v) => match v.check_input() {
                    Ok(_) => Ok(Self { inner_item: v }),
                    Err(e) => Err(e),
                },
                //json解析错误
                Err(e) => Err(ApiError::BadRequest(format!("{}", e))),
            }
        })
    }
}

impl<T: DeserializeOwned + ApiRequestValidator> Deref for ApiRequest<T> {
    type Target = Json<T>;

    fn deref(&self) -> &Self::Target {
        &self.inner_item
    }
}
