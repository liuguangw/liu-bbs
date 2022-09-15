///计数器key定义
pub enum CounterKey {
    ///最后一个注册用户id
    LastUserId,
    ///最后一个论坛分区id
    LastForumGroupId,
    ///最后一个论坛id
    LastForumId,
}

impl CounterKey {
    ///字符串形式
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::LastUserId => "users.last_id",
            Self::LastForumGroupId => "forum_groups.last_id",
            Self::LastForumId => "forums.last_id",
        }
    }
}
