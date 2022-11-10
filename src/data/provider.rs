use super::{
    CounterRepository, ForumRepository, ForumTreeRepository, SessionRepository,
    TopicContentRepository, TopicRepository, UserRepository,
};
use crate::common::DatabaseData;
use std::sync::Arc;

///data provider
pub struct Provider {
    ///session数据操作
    pub session_repo: SessionRepository,
    ///计数器数据操作
    pub counter_repo: CounterRepository,
    ///用户数据操作
    pub user_repo: UserRepository,
    ///论坛树数据操作
    pub forum_tree_repo: ForumTreeRepository,
    ///论坛数据操作
    pub forum_repo: ForumRepository,
    ///帖子数据操作
    pub topic_repo: TopicRepository,
    ///帖子内容数据操作
    pub topic_content_repo: TopicContentRepository,
}

impl From<&Arc<DatabaseData>> for Provider {
    fn from(database_data: &Arc<DatabaseData>) -> Self {
        Self {
            session_repo: database_data.into(),
            counter_repo: database_data.into(),
            user_repo: database_data.into(),
            forum_tree_repo: database_data.into(),
            forum_repo: database_data.into(),
            topic_repo: database_data.into(),
            topic_content_repo: database_data.into(),
        }
    }
}
