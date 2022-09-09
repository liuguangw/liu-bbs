use crate::{
    common::ApiError,
    http::{requests::SessionRequest, responses::CaptchaResponse},
    services::{CaptchaService, SessionService},
};
use actix_web::{get, web};

///显示验证码图片
#[get("/captcha/show")]
pub async fn show(
    session_service: web::Data<SessionService>,
    captcha_service: web::Data<CaptchaService>,
    query: web::Query<SessionRequest>,
) -> Result<CaptchaResponse, ApiError> {
    let session = session_service.load_session(&query.id).await?;
    if session.is_none() {
        return Err(ApiError::Common("invalid session id".to_string()));
    }
    let mut session = session.unwrap();
    //todo deal error
    let captcha = captcha_service.build().unwrap();
    session
        .data
        .insert("code".to_string(), captcha.phrase.to_string());
    session_service.save_session(&mut session).await?;
    //show captcha
    Ok(CaptchaResponse::new(captcha.data()))
}
