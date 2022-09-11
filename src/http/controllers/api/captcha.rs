use crate::{
    common::{ApiError, ApiRequest},
    http::requests::SessionRequest,
    services::{CaptchaService, SessionService},
};
use actix_web::{get, http::header, rt::task, web, CustomizeResponder, Responder};

///显示验证码图片
#[get("/captcha/show")]
pub async fn show(
    session_service: web::Data<SessionService>,
    captcha_service: web::Data<CaptchaService>,
    query: ApiRequest<web::Query<SessionRequest>>,
) -> Result<CustomizeResponder<Vec<u8>>, ApiError> {
    let mut session = match session_service.load_session(&query.session_id).await? {
        Some(v) => v,
        None => return Err(ApiError::InvalidSessionID),
    };
    let captcha_task = task::spawn_blocking(move || {
        // This is running on a blocking thread.
        // Blocking here is ok.
        captcha_service.build()
    });
    let captcha = captcha_task.await.unwrap()?;
    session.data.insert("code".to_string(), captcha.phrase);
    session_service.update_session(&session).await?;
    //show captcha
    let captcha_data = captcha.raw_data;
    let resp = captcha_data
        .customize()
        .insert_header((header::CONTENT_TYPE, "image/png"))
        .insert_header((
            header::CACHE_CONTROL,
            "private, max-age=0, no-store, no-cache, must-revalidate",
        ));
    Ok(resp)
}
