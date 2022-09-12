use crate::{common::ApiError, models::Session, services::SessionService};
use actix_web::{web, FromRequest};
use serde::Deserialize;
use std::{future::Future, pin::Pin};

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
pub struct SessionRequest(pub Session);

impl FromRequest for SessionRequest {
    type Error = ApiError;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(
        req: &actix_web::HttpRequest,
        payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        let session_service = req.app_data::<web::Data<SessionService>>().unwrap().clone();
        let query_fut = web::Query::<SessionRequestParam>::from_request(req, payload);
        let fut = async move {
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
                    match session_service.load_session(&session_id).await {
                        Ok(session_opt) => match session_opt {
                            Some(session) => Ok(SessionRequest(session)),
                            None => Err(ApiError::InvalidSessionID),
                        },
                        //数据库错误
                        Err(e) => Err(ApiError::DatabaseError(e)),
                    }
                }
                None => Err(ApiError::InvalidSessionID),
            }
        };
        Box::pin(fut)
    }
}
