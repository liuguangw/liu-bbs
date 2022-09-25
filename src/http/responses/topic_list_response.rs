use serde::Serialize;

///作者信息节点
#[derive(Serialize, Clone)]
pub struct AuthorInfoNode {
    ///用户id
    pub id: i64,
    ///昵称
    pub nickname: String,
    ///头像
    pub avatar: String,
}

///帖子列表节点
#[derive(Serialize)]
pub struct TopicListNode {
    ///帖子id
    pub id: i64,
    ///作者用户id
    pub author_user_id: i64,
    ///作者信息
    pub author_info: AuthorInfoNode,
    ///最后回复的用户id
    pub last_reply_user_id: Option<i64>,
    ///最后回复的用户信息
    pub last_reply_user_info: Option<AuthorInfoNode>,
    ///标题
    pub title: String,
    ///是否已经发布
    pub is_publish: bool,
    ///是否已锁定
    pub is_lock: bool,
    ///是否已屏蔽
    pub is_block: bool,
    ///是否已删除
    pub is_delete: bool,
    ///阅读数
    pub view_count: i64,
    ///回复数
    pub reply_count: i64,
    ///发布时间
    pub publish_at: String,
    ///最后回复时间
    pub last_reply_at: String,
    ///创建时间
    pub created_at: String,
    ///更新时间
    pub updated_at: String,
}

///分页信息
#[derive(Serialize)]
pub struct PaginationInfo {
    ///当前是第几页, n >= 1
    pub current_page: i64,
    ///总共有几页
    pub total_page: i64,
    ///每页最多数据条数
    pub per_page: i64,
}

///帖子列表响应
#[derive(Serialize)]
pub struct TopicListResponse {
    ///帖子列表
    pub topic_list: Vec<TopicListNode>,
    ///分页信息
    pub pagination_info: PaginationInfo,
}
