use super::{CaptchaService, ForumService, SessionService, TopicService, UserService};
use crate::{
    common::DatabaseData,
    data::{
        CounterRepository, ForumRepository, ForumTreeRepository, SessionRepository,
        TopicContentRepository, TopicRepository, UserRepository,
    },
};
use std::sync::Arc;

///service provider
pub struct Provider {
    ///验证码服务
    pub captcha_service: CaptchaService,
    ///会话服务
    pub session_service: SessionService,
    ///用户相关服务
    pub user_service: UserService,
    ///帖子相关服务
    pub topic_service: TopicService,
    ///论坛相关服务
    pub forum_service: ForumService,
}
impl Provider {
    ///构造函数
    pub fn new(database_data: &Arc<DatabaseData>) -> Self {
        let captcha_service = CaptchaService::default();
        //
        let session_repo = SessionRepository::new(database_data);
        let session_service = SessionService::new(session_repo);
        //
        let counter_repo = Arc::new(CounterRepository::new(database_data));
        let user_repo = UserRepository::new(database_data);
        let user_service = UserService::new(user_repo, &counter_repo);
        //
        let forum_repo = Arc::new(ForumRepository::new(database_data));
        let forum_tree_repo = ForumTreeRepository::new(database_data);
        let topic_repo = TopicRepository::new(database_data);
        let topic_content_repo = TopicContentRepository::new(database_data);
        let topic_service =
            TopicService::new(&counter_repo, &forum_repo, topic_repo, topic_content_repo);
        //
        let forum_service = ForumService::new(&forum_repo, forum_tree_repo);
        //
        Self {
            captcha_service,
            session_service,
            user_service,
            topic_service,
            forum_service,
        }
    }
}
