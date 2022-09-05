use serde::{Deserialize, Serialize};
use std::time::SystemTime;

///数据迁移记录
#[derive(Debug, Serialize, Deserialize)]
pub struct MigrationLog {
    ///迁移记录ID
    #[serde(rename = "_id")]
    pub id: i64,
    ///迁移唯一标识
    pub name: String,
    ///批次序号
    pub batch: i32,
    ///迁移时间
    #[serde(with = "crate::common::serde_helpers::bson_system_time")]
    pub created_at: SystemTime,
}
