use crate::{
    common::{ApiError, DatabaseError},
    data::ForumRepository,
    models::{Forum, ForumGroup},
};
use std::sync::Arc;

///论坛相关服务
pub struct ForumService {
    forum_repo: Arc<ForumRepository>,
}
impl ForumService {
    ///构造函数
    pub fn new(forum_repo: &Arc<ForumRepository>) -> Self {
        Self {
            forum_repo: forum_repo.clone(),
        }
    }
    ///通过论坛id加载论坛信息
    pub async fn load_forum_info_by_id(&self, forum_id: i64) -> Result<Forum, ApiError> {
        let option_forum_info = self
            .forum_repo
            .find_by_id(forum_id)
            .await
            .map_err(DatabaseError::from)?;
        match option_forum_info {
            Some(forum_info) => Ok(forum_info),
            None => {
                let message = format!("forum: {} not found", forum_id);
                Err(ApiError::Common(message))
            }
        }
    }
    ///通过论坛分区id加载分区信息
    pub async fn load_forum_group_info_by_id(
        &self,
        forum_group_id: i64,
    ) -> Result<ForumGroup, ApiError> {
        let option_forum_group_info = self
            .forum_repo
            .find_group_by_id(forum_group_id)
            .await
            .map_err(DatabaseError::from)?;
        match option_forum_group_info {
            Some(forum_info) => Ok(forum_info),
            None => {
                let message = format!("forum group: {} not found", forum_group_id);
                Err(ApiError::Common(message))
            }
        }
    }
}
