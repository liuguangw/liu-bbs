use crate::{
    common::{ApiError, CounterKey, DatabaseError},
    data::{CounterRepository, ForumRepository, TopicContentRepository, TopicRepository},
    models::{Topic, TopicContent},
};
use std::sync::Arc;

///帖子相关服务
pub struct TopicService {
    counter_repo: Arc<CounterRepository>,
    forum_repo: ForumRepository,
    topic_repo: TopicRepository,
    topic_content_repo: TopicContentRepository,
}

impl TopicService {
    ///构造函数
    pub fn new(
        counter_repo: &Arc<CounterRepository>,
        forum_repo: ForumRepository,
        topic_repo: TopicRepository,
        topic_content_repo: TopicContentRepository,
    ) -> Self {
        Self {
            counter_repo: counter_repo.clone(),
            forum_repo,
            topic_repo,
            topic_content_repo,
        }
    }

    ///处理发帖或保存帖子为草稿
    pub async fn process_post_topic(
        &self,
        topic_info: &mut Topic,
        topic_content: &mut TopicContent,
    ) -> Result<(), ApiError> {
        //判断论坛是否存在
        let forum_id = topic_info.forum_id;
        let forum_info = self
            .forum_repo
            .find_by_id(forum_id)
            .await
            .map_err(DatabaseError::from)?;
        if forum_info.is_none() {
            let message = format!("forum {} not found", forum_id);
            return Err(ApiError::Common(message));
        }
        //计算帖子id
        let topic_id = self
            .counter_repo
            .increment(CounterKey::LastTopicId)
            .await
            .map_err(DatabaseError::from)?;
        topic_info.id = topic_id;
        topic_content.id = topic_id;
        //保存帖子到数据库
        self.topic_repo
            .insert_topic(topic_info)
            .await
            .map_err(DatabaseError::from)?;
        self.topic_content_repo
            .insert_topic(topic_content)
            .await
            .map_err(DatabaseError::from)?;
        //更新论坛的发帖数量
        if topic_info.is_publish {
            self.forum_repo
                .incr_topic_count(forum_id, 1)
                .await
                .map_err(DatabaseError::from)?;
        }
        Ok(())
    }
}
