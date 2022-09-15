use crate::{common::ApiError, models::Session, services::SessionService};
use actix_web::{web, FromRequest};
use serde::Deserialize;
use std::{future::Future, pin::Pin, time::SystemTime};

///附带会话id的请求
#[derive(Deserialize, Default)]
#[serde(default)]
pub struct SessionRequestParam {
    ///会话id
    #[serde(rename = "sid")]
    pub session_id: String,
}

///验证session有效的请求
///
/// sid通过url参数传递
/// 如果sid为空，或者sid在数据库内不存在,则返回 [`ApiError::InvalidSessionID`]
pub struct SessionRequest(Session);

impl SessionRequest {
    ///获取session
    pub fn into_inner(self) -> Session {
        self.0
    }
    ///load_session
    async fn load_session_from_request(
        session_service: web::Data<SessionService>,
        query_fut: <web::Query<SessionRequestParam> as FromRequest>::Future,
    ) -> Result<Self, ApiError> {
        //从query中获取SessionRequestParam
        let request_params_opt = match query_fut.await {
            Ok(t) => {
                if t.session_id.is_empty() {
                    None
                } else {
                    Some(t.0)
                }
            }
            Err(_) => None,
        };
        match request_params_opt {
            Some(params) => {
                let session_id = params.session_id;
                //从数据库中加载session
                let session = session_service
                    .load_session(&session_id)
                    .await?
                    //map None as ApiError::InvalidSessionID
                    .ok_or(ApiError::InvalidSessionID)?;
                //判断是否已经过期
                let time_now = SystemTime::now();
                //expired_at <= time_now
                if session.expired_at.le(&time_now) {
                    Err(ApiError::InvalidSessionID)
                } else {
                    Ok(SessionRequest(session))
                }
            }
            None => Err(ApiError::InvalidSessionID),
        }
    }
}

impl FromRequest for SessionRequest {
    type Error = ApiError;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(
        req: &actix_web::HttpRequest,
        payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        let session_service = req.app_data::<web::Data<SessionService>>().unwrap().clone();
        let query_fut = web::Query::<SessionRequestParam>::from_request(req, payload);
        let fut = Self::load_session_from_request(session_service, query_fut);
        Box::pin(fut)
    }
}
