use serde::Serialize;

///初始化session的响应
#[derive(Serialize)]
pub struct InitSessionResponse {
    ///会话唯一标识
    #[serde(rename = "sid")]
    pub session_id: String,
    ///有效期(单位:秒)
    pub expires_in: u64,
}
