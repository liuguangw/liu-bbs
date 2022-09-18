///集合名称
pub enum CollectionName {
    ///迁移记录集合
    Migrations,
    ///计数器集合
    Counters,
    ///用户集合
    Users,
    ///用户邮箱绑定记录集合
    UserEmails,
    ///用户会话集合
    Sessions,
    ///论坛分区集合
    ForumGroups,
    ///论坛集合
    Forums,
    ///论坛树结构
    ForumTrees,
    ///帖子集合
    Topics,
    /// 帖子内容集合
    TopicContents,
    ///帖子回复集合
    TopicReplies,
}

impl CollectionName {
    ///字符串形式
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Migrations => "migrations",
            Self::Counters => "counters",
            Self::Users => "users",
            Self::UserEmails => "user_emails",
            Self::Sessions => "sessions",
            Self::ForumGroups => "forum_groups",
            Self::Forums => "forums",
            Self::ForumTrees => "forum_trees",
            Self::Topics => "topics",
            Self::TopicContents => "topic_contents",
            Self::TopicReplies => "topic_replies",
        }
    }
}
