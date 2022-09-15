use crate::{
    common::{ApiRequest, ResponseResult},
    http::{
        requests::{LoginRequest, SessionRequest},
        responses::LoginResponse,
    },
    services::{SessionService, UserService},
};
use actix_web::{
    post,
    web::{self, Json},
};

///用户登录
#[post("/auth/login")]
pub async fn login(
    req: ApiRequest<Json<LoginRequest>>,
    session_req: SessionRequest,
    session_service: web::Data<SessionService>,
    user_service: web::Data<UserService>,
) -> ResponseResult<LoginResponse> {
    let mut session = session_req.into_inner();
    //检测验证码
    session_service
        .verify_captcha_code(&mut session, &req.captcha_code)
        .await?;
    //process login
    let user = user_service
        .process_login(&req.username, &req.password)
        .await?;
    let session = session_service.create_new_session(Some(user.id)).await?;
    let resp = LoginResponse::from(session);
    Ok(resp.into())
}
