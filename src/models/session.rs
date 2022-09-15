use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    ops::Add,
    time::{Duration, SystemTime},
};

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
    #[serde(with = "crate::common::serde_helpers::bson_system_time")]
    pub created_at: SystemTime,
    ///过期时间
    #[serde(with = "crate::common::serde_helpers::bson_system_time")]
    pub expired_at: SystemTime,
}

impl Default for Session {
    fn default() -> Self {
        let created_at = SystemTime::now();
        let expired_at = created_at.add(Duration::from_secs(30 * 60));
        Self {
            id: Default::default(),
            user_id: Default::default(),
            data: Default::default(),
            created_at,
            expired_at,
        }
    }
}

impl Session {
    ///为登录用户创建一个新的session对象
    pub fn new(user_id: i64) -> Self {
        let created_at = SystemTime::now();
        let duration_secs = 30 * 24 * 3600;
        let expired_at = created_at.add(Duration::from_secs(duration_secs));
        Self {
            user_id,
            created_at,
            expired_at,
            ..Default::default()
        }
    }
}
