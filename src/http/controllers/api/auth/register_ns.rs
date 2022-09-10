use crate::{
    common::{ApiError, ResponseResult},
    http::{requests::RegisterRequest, responses::LoginResponse},
    models::User,
    services::{SessionService, UserService},
};
use actix_web::{dev::ConnectionInfo, post, web};
use std::time::SystemTime;

///用户注册
#[post("/auth/register")]
pub async fn register(
    session_service: web::Data<SessionService>,
    user_service: web::Data<UserService>,
    conn: ConnectionInfo,
    req: web::Json<RegisterRequest>,
) -> ResponseResult<LoginResponse> {
    req.check_input()?;
    let mut session = match session_service.load_session(&req.session_id).await? {
        Some(v) => v,
        None => return Err(ApiError::InvalidSessionID),
    };
    //检测验证码
    let code_phrase = session.data.remove("code");
    session_service.update_session(&session).await?;
    match code_phrase {
        Some(phrase) => {
            if !phrase.eq_ignore_ascii_case(&req.captcha_code) {
                return Err(ApiError::new_bad_request("验证码错误"));
            }
        }
        None => return Err(ApiError::new_bad_request("无效的验证码")),
    };
    //初始化user
    let mut user = User::default();
    user.username = req.username.to_string();
    user.nickname = req.nickname.to_string();
    user.set_password(&req.password);
    user.register_ip = conn.realip_remote_addr().unwrap().to_string();
    //process register
    user_service.process_register(&mut user).await?;
    let session = session_service.create_new_session(Some(user.id)).await?;
    let expires_in = session
        .expired_at
        .duration_since(SystemTime::now())
        .unwrap();
    let resp = LoginResponse {
        user_id: user.id,
        session_id: session.id,
        expires_in: expires_in.as_secs(),
    };
    Ok(resp.into())
}
