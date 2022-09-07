use serde::{Deserialize, Serialize};

///初始化session的响应
#[derive(Serialize, Deserialize)]
pub struct InitSessionResponse {
    ///会话唯一标识
    pub id: String,
    ///有效期(单位:秒)
    pub expires_in: u64,
}
