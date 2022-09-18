///计数器key定义
pub enum CounterKey {
    ///最后一个注册用户id
    LastUserId,
    ///最后一个论坛分区id
    LastForumGroupId,
    ///最后一个论坛id
    LastForumId,
    ///最后一个帖子id
    LastTopicId,
    ///最后一个回帖id
    LastTopicReplyId,
}

impl CounterKey {
    ///字符串形式
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::LastUserId => "users.last_id",
            Self::LastForumGroupId => "forum_groups.last_id",
            Self::LastForumId => "forums.last_id",
            Self::LastTopicId => "topics.last_id",
            Self::LastTopicReplyId => "topic_replies.last_id",
        }
    }
}
