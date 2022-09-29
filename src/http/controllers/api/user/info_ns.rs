use crate::{
    common::ResponseResult,
    http::{requests::AuthSessionRequest, responses::UserInfoSelfResponse},
    services::Provider,
};
use actix_web::{get, web};

///获取当前登录的用户信息
#[get("/user/info")]
pub async fn info(
    session_req: AuthSessionRequest,
    service_provider: web::Data<Provider>,
) -> ResponseResult<UserInfoSelfResponse> {
    let user_id = session_req.user_id;
    let user_info = service_provider
        .user_service
        .load_user_info_by_id(user_id)
        .await?;
    let resp = UserInfoSelfResponse::from(user_info);
    Ok(resp.into())
}
