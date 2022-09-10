use crate::{
    common::ApiError,
    http::requests::SessionRequest,
    services::{CaptchaService, SessionService},
};
use actix_web::{get, http::header, web, CustomizeResponder, Responder};

///显示验证码图片
#[get("/captcha/show")]
pub async fn show(
    session_service: web::Data<SessionService>,
    captcha_service: web::Data<CaptchaService>,
    query: web::Query<SessionRequest>,
) -> Result<CustomizeResponder<Vec<u8>>, ApiError> {
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
    let captcha_data = Vec::from(captcha.data());
    let resp = captcha_data
        .customize()
        .insert_header((header::CONTENT_TYPE, "image/png"))
        .insert_header((
            header::CACHE_CONTROL,
            "private, max-age=0, no-store, no-cache, must-revalidate",
        ));
    Ok(resp)
}
