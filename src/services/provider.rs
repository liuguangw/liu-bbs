use super::{CaptchaService, ForumService, SessionService, TopicService, UserService};
use crate::{common::DatabaseData, data::Provider as DataProvider};
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
        let data_provider = Arc::new(DataProvider::from(database_data));
        //
        Self {
            captcha_service: CaptchaService::default(),
            session_service: (&data_provider).into(),
            user_service: (&data_provider).into(),
            topic_service: (&data_provider).into(),
            forum_service: (&data_provider).into(),
        }
    }
}
