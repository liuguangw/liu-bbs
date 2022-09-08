use crate::{
    common::{ApiError, ResponseResult},
    http::requests::SessionRequest,
    services::SessionService,
};
use actix_web::{get, web};

///显示验证码图片
#[get("/captcha/show")]
pub async fn show(
    session_service: web::Data<SessionService>,
    query: web::Query<SessionRequest>,
) -> ResponseResult<String> {
    let session = session_service.load_session(&query.id).await?;
    if session.is_none() {
        return Err(ApiError::Common("invalid session id".to_string()));
    }
    let session = session.unwrap();
    //todo show captcha
    Ok(session.id.into())
}
