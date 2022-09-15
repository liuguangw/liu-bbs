use crate::{
    common::ApiError,
    http::requests::SessionRequest,
    services::{CaptchaService, SessionService},
};
use actix_web::{get, http::header, rt::task, web, CustomizeResponder, Responder};

///显示验证码图片
#[get("/captcha/show")]
pub async fn show(
    session_req: SessionRequest,
    session_service: web::Data<SessionService>,
    captcha_service: web::Data<CaptchaService>,
) -> Result<CustomizeResponder<Vec<u8>>, ApiError> {
    let captcha_task = task::spawn_blocking(move || {
        // This is running on a blocking thread.
        // Blocking here is ok.
        captcha_service.build()
    });
    let captcha = captcha_task.await.unwrap()?;
    let mut session = session_req.into_inner();
    session_service
        .set_captcha_code(&mut session, captcha.phrase)
        .await?;
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
