use crate::models::Session;
use serde::Serialize;
use std::time::SystemTime;

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

impl From<Session> for LoginResponse {
    fn from(session: Session) -> Self {
        let expires_in = session
            .expired_at
            .duration_since(SystemTime::now())
            .unwrap();
        Self {
            user_id: session.user_id,
            session_id: session.id,
            expires_in: expires_in.as_secs(),
        }
    }
}
