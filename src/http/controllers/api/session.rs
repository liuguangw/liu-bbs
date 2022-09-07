use crate::{common::ResponseResult, http::responses::InitSessionResponse, models::Session};
use actix_web::post;
use std::time::SystemTime;

///初始化用户会话
#[post("/session/init")]
pub async fn init_session() -> ResponseResult<InitSessionResponse> {
    //demo data
    let session = Session::default();
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
