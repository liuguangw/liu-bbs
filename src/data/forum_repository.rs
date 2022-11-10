use crate::{
    common::{CollectionName, DatabaseData},
    models::{Forum, ForumGroup},
};
use mongodb::{
    bson::doc,
    options::{FindOneAndUpdateOptions, ReturnDocument},
    Collection,
};
use std::sync::Arc;

///forum repository
pub struct ForumRepository {
    database_data: Arc<DatabaseData>,
}

impl From<&Arc<DatabaseData>> for ForumRepository {
    fn from(database_data: &Arc<DatabaseData>) -> Self {
        Self {
            database_data: database_data.clone(),
        }
    }
}

impl ForumRepository {
    fn collection(&self) -> Collection<Forum> {
        self.database_data.collection(CollectionName::Forums)
    }

    fn group_collection(&self) -> Collection<ForumGroup> {
        self.database_data.collection(CollectionName::ForumGroups)
    }

    ///根据id查找论坛
    pub async fn find_by_id(&self, id: i64) -> mongodb::error::Result<Option<Forum>> {
        let filter = doc! {
            "_id": id,
        };
        let coll = self.collection();
        coll.find_one(filter, None).await
    }

    ///根据id查找论坛分区
    pub async fn find_group_by_id(&self, id: i64) -> mongodb::error::Result<Option<ForumGroup>> {
        let filter = doc! {
            "_id": id,
        };
        let coll = self.group_collection();
        coll.find_one(filter, None).await
    }

    ///增加或者减少论坛帖子计数,返回更新后的计数
    pub async fn incr_topic_count(
        &self,
        forum_id: i64,
        incr_value: i64,
    ) -> mongodb::error::Result<i64> {
        let update_data = doc! {
            "$inc": doc! {
                "topic_count": incr_value,
            }
        };
        let filter = doc! {
            "_id": forum_id,
        };
        let opt = FindOneAndUpdateOptions::builder()
            .return_document(ReturnDocument::After)
            .upsert(true)
            .build();
        let coll = self.collection();
        let item = coll.find_one_and_update(filter, update_data, opt).await?;
        Ok(item.unwrap().topic_count)
    }
}
