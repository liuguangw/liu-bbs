use crate::{
    common::{ApiError, ResponseResult},
    http::{requests::SessionRequest, responses::UserInfoSelfResponse},
    services::UserService,
};
use actix_web::{get, web};

///获取当前登录的用户信息
#[get("/user/info")]
pub async fn info(
    user_service: web::Data<UserService>,
    session_req: SessionRequest,
) -> ResponseResult<UserInfoSelfResponse> {
    let session = session_req.0;
    let user_id = session.user_id;
    if user_id == 0 {
        return Err(ApiError::UserNotLogin);
    }
    let user_info = user_service.load_user_info_by_id(user_id).await?;
    let resp = UserInfoSelfResponse::from(user_info);
    Ok(resp.into())
}
