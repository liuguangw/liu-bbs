use std::sync::Arc;

use futures::TryStreamExt;
use mongodb::{bson::doc, options::FindOptions, Collection};

use crate::{
    common::{CollectionName, DatabaseData},
    models::ForumTree,
};

///forum tree repository
pub struct ForumTreeRepository {
    database_data: Arc<DatabaseData>,
}

impl ForumTreeRepository {
    ///构造函数
    pub fn new(database_data: &Arc<DatabaseData>) -> Self {
        Self {
            database_data: database_data.clone(),
        }
    }

    fn collection(&self) -> Collection<ForumTree> {
        self.database_data.collection(CollectionName::ForumTrees)
    }

    ///获取上级论坛结构树列表
    pub async fn load_parent_trees(&self, forum_id: i64) -> mongodb::error::Result<Vec<ForumTree>> {
        let filter = doc! {
            "child_forum_id" : forum_id,
        };
        let sort_opt = doc! {
                "path_deep" : -1,
        };
        let find_options = FindOptions::builder().sort(sort_opt).build();
        let coll = self.collection();
        let cursor = coll.find(filter, find_options).await?;
        cursor.try_collect().await
    }
}
