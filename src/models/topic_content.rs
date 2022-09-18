use serde::{Deserialize, Serialize};

///帖子内容
#[derive(Serialize, Deserialize)]
pub struct TopicContent {
    ///帖子id
    #[serde(rename = "_id")]
    pub id: i64,
    ///帖子内容
    pub content: String,
}
