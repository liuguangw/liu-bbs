use serde::Serialize;

///登录成功的响应
#[derive(Serialize)]
pub struct LoginResponse {
    ///用户id
    pub user_id: i64,
    ///会话id
    #[serde(rename = "sid")]
    pub session_id: String,
    ///有效期(单位:秒)
    pub expires_in: u64,
}
