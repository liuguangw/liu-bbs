use serde::{Deserialize, Serialize};
use std::time::SystemTime;

///帖子
#[derive(Serialize, Deserialize)]
pub struct Topic {
    ///帖子id
    #[serde(rename = "_id")]
    pub id: i64,
    ///作者用户id
    pub author_user_id: i64,
    ///最后回复的用户id
    pub last_reply_user_id: Option<i64>,
    ///论坛id
    pub forum_id: i64,
    ///标题
    pub title: String,
    ///是否已经发布
    pub is_publish: bool,
    ///是否已锁定
    pub is_lock: bool,
    ///是否已屏蔽
    pub is_block: bool,
    ///是否已删除
    pub is_delete: bool,
    ///阅读数
    pub view_count: i64,
    ///回复数
    pub reply_count: i64,
    ///发布时间
    #[serde(with = "crate::common::serde_helpers::bson_option_system_time")]
    pub publish_at: Option<SystemTime>,
    ///最后回复时间
    #[serde(with = "crate::common::serde_helpers::bson_option_system_time")]
    pub last_reply_at: Option<SystemTime>,
    ///创建时间
    #[serde(with = "crate::common::serde_helpers::bson_system_time")]
    pub created_at: SystemTime,
    ///更新时间
    #[serde(with = "crate::common::serde_helpers::bson_system_time")]
    pub updated_at: SystemTime,
}
