use crate::{
    common::ResponseResult, http::responses::InitSessionResponse, services::SessionService,
};
use actix_web::{post, web};
use std::time::SystemTime;

///初始化用户会话
#[post("/session/init")]
pub async fn init_session(
    session_service: web::Data<SessionService>,
) -> ResponseResult<InitSessionResponse> {
    //demo data
    let session = session_service.create_new_session(None).await?;
    let expires_in = session
        .expired_at
        .duration_since(SystemTime::now())
        .unwrap();
    let response = InitSessionResponse {
        id: session.id,
        expires_in: expires_in.as_secs(),
    };
    Ok(response.into())
}
