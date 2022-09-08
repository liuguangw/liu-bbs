use std::time::SystemTime;

use serde::{Deserialize, Serialize};

///用户邮箱绑定记录
#[derive(Serialize, Deserialize)]
pub struct UserEmail {
    ///用户id
    pub user_id: i64,
    ///email地址
    pub email: String,
    ///绑定时间
    #[serde(with = "crate::common::serde_helpers::bson_system_time")]
    pub created_at: SystemTime,
}
