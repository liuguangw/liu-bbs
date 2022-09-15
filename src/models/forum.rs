use serde::{Deserialize, Serialize};
use std::time::SystemTime;

///论坛
#[derive(Serialize, Deserialize)]
pub struct Forum {
    ///论坛id
    #[serde(rename = "_id")]
    pub id: i64,
    ///分区id
    pub forum_group_id: i64,
    ///名称
    pub name: String,
    ///简介
    pub description: String,
    ///图标
    pub icon_url: String,
    ///主题帖总数量
    pub topic_count: i64,
    ///回帖总数量
    pub reply_count: i64,
    ///排序值
    pub order_id: i32,
    ///创建时间
    #[serde(with = "crate::common::serde_helpers::bson_system_time")]
    pub created_at: SystemTime,
    ///更新时间
    #[serde(with = "crate::common::serde_helpers::bson_system_time")]
    pub updated_at: SystemTime,
}
