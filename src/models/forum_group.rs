use serde::{Deserialize, Serialize};
use std::time::SystemTime;

///分区
#[derive(Serialize, Deserialize)]
pub struct ForumGroup {
    ///分区id
    #[serde(rename = "_id")]
    pub id: i64,
    ///名称
    pub name: String,
    ///简介
    pub description: String,
    ///图标
    pub icon_url: String,
    ///排序值
    pub order_id: i32,
    ///创建时间
    #[serde(with = "crate::common::serde_helpers::bson_system_time")]
    pub created_at: SystemTime,
    ///更新时间
    #[serde(with = "crate::common::serde_helpers::bson_system_time")]
    pub updated_at: SystemTime,
}
