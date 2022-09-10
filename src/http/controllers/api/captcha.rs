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
    let mut session = match session_service.load_session(&query.id).await? {
        Some(v) => v,
        None => return Err(ApiError::InvalidSessionID),
    };
    let captcha = captcha_service.build()?;
    session
        .data
        .insert("code".to_string(), captcha.phrase.to_string());
    session_service.update_session(&session).await?;
    //show captcha
    Ok(CaptchaResponse::new(captcha.data()))
}
