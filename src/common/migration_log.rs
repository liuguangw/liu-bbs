use serde::{Deserialize, Serialize};
use std::time::SystemTime;

///数据迁移记录
#[derive(Debug, Serialize, Deserialize)]
pub struct MigrationLog {
    pub id: i64,
    pub name: String,
    pub batch: i32,
    pub created_at: SystemTime,
}
