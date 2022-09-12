use super::SessionRequest;
use crate::{common::ApiError, models::Session};
use actix_web::FromRequest;
use std::future::Future;
use std::{ops::Deref, pin::Pin};

///验证session已登录的请求
///
/// sid通过url参数传递
/// 如果sid为空，或者sid在数据库内不存在,或者[`Session`]中的用户id为0,都会返回 [`ApiError::UserNotLogin`]
pub struct AuthSessionRequest(Session);

impl AuthSessionRequest {
    ///获取session
    pub fn get_inner(self) -> Session {
        self.0
    }
}

impl Deref for AuthSessionRequest {
    type Target = Session;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromRequest for AuthSessionRequest {
    type Error = ApiError;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(
        req: &actix_web::HttpRequest,
        payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        let session_req_mut = SessionRequest::from_request(req, payload);
        let fut = async move {
            match session_req_mut.await {
                Ok(session_req) => {
                    let session = session_req.get_inner();
                    //用户id不能为0
                    if session.user_id == 0 {
                        Err(ApiError::UserNotLogin)
                    } else {
                        Ok(AuthSessionRequest(session))
                    }
                }
                Err(e) => {
                    if let ApiError::InvalidSessionID = e {
                        //InvalidSessionID -> UserNotLogin
                        Err(ApiError::UserNotLogin)
                    } else {
                        //其他错误不变(例如数据库出错)
                        Err(e)
                    }
                }
            }
        };
        Box::pin(fut)
    }
}
