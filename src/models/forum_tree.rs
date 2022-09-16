use serde::{Deserialize, Serialize};
///论坛树结构
///
/// (forum_id,child_forum_id)保持unique限制
#[derive(Serialize, Deserialize)]
pub struct ForumTree {
    ///论坛id
    pub forum_id: i64,
    ///子论坛id
    pub child_forum_id: i64,
    ///关系深度
    pub path_deep: u8,
}
