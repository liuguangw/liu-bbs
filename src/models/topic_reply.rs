use serde::{Deserialize, Serialize};
use std::time::SystemTime;

///回帖
#[derive(Serialize, Deserialize)]
pub struct TopicReply {
    ///回复id
    #[serde(rename = "_id")]
    pub id: i64,
    ///主题帖id
    pub topic_id: i64,
    ///楼层号
    pub floor_number: i64,
    ///回帖用户id
    pub author_user_id: i64,
    ///内容
    pub content: String,
    ///是否已屏蔽
    pub is_block: bool,
    ///是否已删除
    pub is_delete: bool,
    ///创建时间
    #[serde(with = "crate::common::serde_helpers::bson_system_time")]
    pub created_at: SystemTime,
    ///更新时间
    #[serde(with = "crate::common::serde_helpers::bson_system_time")]
    pub updated_at: SystemTime,
}
