use crate::{
    common::{ApiError, CounterKey, DatabaseError, DatabaseResult},
    data::{CounterRepository, ForumRepository, TopicContentRepository, TopicRepository},
    models::{Topic, TopicContent},
};
use std::sync::Arc;

///帖子相关服务
pub struct TopicService {
    counter_repo: Arc<CounterRepository>,
    forum_repo: Arc<ForumRepository>,
    topic_repo: TopicRepository,
    topic_content_repo: TopicContentRepository,
}

impl TopicService {
    ///构造函数
    pub fn new(
        counter_repo: &Arc<CounterRepository>,
        forum_repo: &Arc<ForumRepository>,
        topic_repo: TopicRepository,
        topic_content_repo: TopicContentRepository,
    ) -> Self {
        Self {
            counter_repo: counter_repo.clone(),
            forum_repo: forum_repo.clone(),
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
                .incr_topic_count(topic_info.forum_id, 1)
                .await
                .map_err(DatabaseError::from)?;
        }
        Ok(())
    }

    ///获取论坛帖子列表
    pub async fn get_forum_topic_list(
        &self,
        forum_id: i64,
        sort_type: u8,
        offset: u64,
        limit: i64,
    ) -> DatabaseResult<Vec<Topic>> {
        self.topic_repo
            .get_forum_topic_list(forum_id, sort_type, offset, limit)
            .await
            .map_err(DatabaseError::from)
    }

    ///计算帖子总数
    pub async fn get_forum_topic_count(&self, forum_id: i64) -> DatabaseResult<u64> {
        self.topic_repo
            .get_forum_topic_count(forum_id)
            .await
            .map_err(DatabaseError::from)
    }

    ///通过帖子id加载帖子信息
    pub async fn load_topic_info_by_id(&self, topic_id: i64) -> DatabaseResult<Option<Topic>> {
        self.topic_repo
            .find_by_id(topic_id)
            .await
            .map_err(DatabaseError::from)
    }

    ///通过帖子id加载帖子内容
    pub async fn load_topic_content_by_id(
        &self,
        topic_id: i64,
    ) -> DatabaseResult<Option<TopicContent>> {
        self.topic_content_repo
            .find_by_id(topic_id)
            .await
            .map_err(DatabaseError::from)
    }
}
