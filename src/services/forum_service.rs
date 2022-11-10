use crate::{
    common::{ApiError, DatabaseError},
    data::{ForumRepository, ForumTreeRepository, Provider as DataProvider},
    http::responses::CommonNodeResponse,
    models::{Forum, ForumGroup},
};
use std::sync::Arc;

///论坛相关服务
pub struct ForumService {
    data_provider: Arc<DataProvider>,
}

impl From<&Arc<DataProvider>> for ForumService {
    fn from(item: &Arc<DataProvider>) -> Self {
        Self {
            data_provider: item.clone(),
        }
    }
}

impl ForumService {
    #[inline]
    fn forum_repo(&self) -> &ForumRepository {
        &self.data_provider.forum_repo
    }

    #[inline]
    fn forum_tree_repo(&self) -> &ForumTreeRepository {
        &self.data_provider.forum_tree_repo
    }
    ///通过论坛id加载论坛信息
    pub async fn load_forum_info_by_id(&self, forum_id: i64) -> Result<Forum, ApiError> {
        let option_forum_info = self
            .forum_repo()
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
            .forum_repo()
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

    ///获取上级论坛列表
    pub async fn load_parent_forums(
        &self,
        forum_id: i64,
    ) -> Result<Vec<CommonNodeResponse>, ApiError> {
        let parent_forum_trees = self
            .forum_tree_repo()
            .load_parent_trees(forum_id)
            .await
            .map_err(DatabaseError::from)?;
        let mut item_list = Vec::new();
        for tree_node in parent_forum_trees {
            let forum_info = self.load_forum_info_by_id(tree_node.forum_id).await?;
            let forum_info_node = CommonNodeResponse {
                id: forum_info.id,
                name: forum_info.name,
                description: forum_info.description,
            };
            item_list.push(forum_info_node);
        }
        Ok(item_list)
    }
}
