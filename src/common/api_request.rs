use super::ApiError;
use actix_web::{web, FromRequest};
use futures_util::ready;
use std::{
    future::Future,
    ops::Deref,
    pin::Pin,
    task::{Context, Poll},
};

///检测用户的输入请求
pub trait ApiRequestValidator {
    ///检测用户输入
    fn check_input(&self) -> Result<(), ApiError>;
}

///api请求封装
///
/// 如果需要验证用户的输入，那么 `T` 需要实现 [`ApiRequestValidator`] 和 [`FromRequest`]
pub struct ApiRequest<T>(pub T);

impl<T> FromRequest for ApiRequest<T>
where
    T: FromRequest + ApiRequestValidator,
{
    type Error = ApiError;
    type Future = ApiRequestFuture<T::Future>;

    fn from_request(
        req: &actix_web::HttpRequest,
        payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        ApiRequestFuture {
            fut: Box::pin(T::from_request(req, payload)),
        }
    }
}

pub struct ApiRequestFuture<Fut> {
    fut: Pin<Box<Fut>>,
}

impl<Fut, T, E> Future for ApiRequestFuture<Fut>
where
    Fut: Future<Output = Result<T, E>>,
    E: Into<actix_web::Error>,
    T: FromRequest + ApiRequestValidator,
{
    type Output = Result<ApiRequest<T>, ApiError>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let p = self.get_mut();
        //Result<T, E>
        let res = ready!(p.fut.as_mut().poll(cx));
        match res {
            Ok(v) => {
                //输入验证
                let check_result = v
                    .check_input()
                    //Result<(),ApiError> => Result<ApiRequest<T>, ApiError>
                    .map(|_| ApiRequest(v));
                Poll::Ready(check_result)
            }
            Err(e) => {
                //解析失败
                let err: actix_web::Error = e.into();
                let message = format!("{}", err);
                //转化为ApiError
                Poll::Ready(Err(ApiError::BadRequest(message)))
            }
        }
    }
}

impl<T> Deref for ApiRequest<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> ApiRequestValidator for web::Json<T>
where
    T: ApiRequestValidator,
{
    fn check_input(&self) -> Result<(), ApiError> {
        self.0.check_input()
    }
}

impl<T> ApiRequestValidator for web::Query<T>
where
    T: ApiRequestValidator,
{
    fn check_input(&self) -> Result<(), ApiError> {
        self.0.check_input()
    }
}
