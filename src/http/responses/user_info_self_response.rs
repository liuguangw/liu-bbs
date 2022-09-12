use serde::Serialize;
use std::time::SystemTime;

use crate::models::User;

///当前用户信息的响应
#[derive(Serialize)]
pub struct UserInfoSelfResponse {
    ///用户id
    pub id: i64,
    ///用户名
    pub username: String,
    ///昵称
    pub nickname: String,
    ///头像
    pub avatar: String,
    ///经验值
    pub exp: i64,
    ///金币值
    pub coin: i64,
    ///已验证邮箱
    pub status_email_verified: bool,
    ///已验证手机
    pub status_mobile_verified: bool,
    ///注册时的IP地址
    pub register_ip: String,
    ///注册时间
    #[serde(serialize_with = "crate::common::serde_helpers::json_system_time::serialize")]
    pub created_at: SystemTime,
    ///更新时间
    #[serde(serialize_with = "crate::common::serde_helpers::json_system_time::serialize")]
    pub updated_at: SystemTime,
}

impl From<User> for UserInfoSelfResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            username: user.username,
            nickname: user.nickname,
            avatar: user.avatar,
            exp: user.exp,
            coin: user.coin,
            status_email_verified: user.status_email_verified,
            status_mobile_verified: user.status_mobile_verified,
            register_ip: user.register_ip,
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }
}
