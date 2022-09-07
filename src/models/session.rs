use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    ops::Add,
    time::{Duration, SystemTime},
};
use uuid::Uuid;

///用户会话
#[derive(Serialize, Deserialize)]
pub struct Session {
    ///唯一标识
    #[serde(rename = "_id")]
    pub id: String,
    ///用户id
    pub user_id: i64,
    ///session数据
    pub data: HashMap<String, String>,
    ///创建时间
    pub created_at: SystemTime,
    ///过期时间
    pub expired_at: SystemTime,
}

impl Default for Session {
    fn default() -> Self {
        //30分钟有效期
        Self::build_new_session(0, 30 * 60)
    }
}
impl Session {
    fn build_new_session(user_id: i64, duration_secs: u64) -> Self {
        let id = Uuid::new_v4().simple();
        let id = id.encode_lower(&mut Uuid::encode_buffer()).to_string();
        let created_at = SystemTime::now();
        let expired_at = created_at.add(Duration::from_secs(duration_secs));
        Self {
            id,
            user_id,
            data: Default::default(),
            created_at,
            expired_at,
        }
    }
    ///为用户创建一个新的session对象
    pub fn new(user_id: i64) -> Self {
        if user_id <= 0 {
            return Self::default();
        }
        Self::build_new_session(user_id, 30 * 24 * 3600)
    }
}
