use crate::{
    common::{ApiRequest, ResponseResult},
    http::{
        requests::{RegisterRequest, SessionRequest},
        responses::LoginResponse,
    },
    models::User,
    services::Provider,
};
use actix_web::{
    dev::ConnectionInfo,
    post,
    web::{self, Json},
};

///用户注册
#[post("/auth/register")]
pub async fn register(
    req: ApiRequest<Json<RegisterRequest>>,
    session_req: SessionRequest,
    conn: ConnectionInfo,
    service_provider: web::Data<Provider>,
) -> ResponseResult<LoginResponse> {
    let mut session = session_req.into_inner();
    //检测验证码
    service_provider
        .session_service
        .verify_captcha_code(&mut session, &req.captcha_code)
        .await?;
    //初始化user
    let mut user = User::default();
    user.username = req.username.to_string();
    user.nickname = req.nickname.to_string();
    user.set_password(&req.password);
    user.register_ip = conn.realip_remote_addr().unwrap().to_string();
    //process register
    service_provider
        .user_service
        .process_register(&mut user)
        .await?;
    let session = service_provider
        .session_service
        .create_new_session(Some(user.id))
        .await?;
    let resp = LoginResponse::from(session);
    Ok(resp.into())
}
