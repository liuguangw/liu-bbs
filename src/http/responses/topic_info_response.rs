use super::TopicListNode;
use serde::Serialize;

///帖子详情页响应
#[derive(Serialize)]
pub struct TopicInfoResponse {
    ///帖子meta信息
    pub meta_info: TopicListNode,
    ///帖子内容
    pub content: String,
    //todo 添加更多字段
}
