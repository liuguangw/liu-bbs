use crate::models::Forum;
use serde::Serialize;

///论坛信息
#[derive(Serialize)]
pub struct ForumInfoResponse {
    ///论坛id
    pub id: i64,
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
}

impl From<Forum> for ForumInfoResponse {
    fn from(forum_info: Forum) -> Self {
        ForumInfoResponse {
            id: forum_info.id,
            name: forum_info.name,
            description: forum_info.description,
            icon_url: forum_info.icon_url,
            topic_count: forum_info.topic_count,
            reply_count: forum_info.reply_count,
        }
    }
}
