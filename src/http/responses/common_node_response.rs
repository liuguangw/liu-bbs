use crate::models::ForumGroup;
use serde::Serialize;

///通用节点信息
#[derive(Serialize)]
pub struct CommonNodeResponse {
    ///id
    pub id: i64,
    ///名称
    pub name: String,
    ///简介
    pub description: String,
}

impl From<ForumGroup> for CommonNodeResponse {
    fn from(item: ForumGroup) -> Self {
        Self {
            id: item.id,
            name: item.name,
            description: item.description,
        }
    }
}
