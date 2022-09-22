use crate::{
    common::{CollectionName, DatabaseData},
    models::Forum,
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

impl ForumRepository {
    ///构造函数
    pub fn new(database_data: &Arc<DatabaseData>) -> Self {
        Self {
            database_data: database_data.clone(),
        }
    }

    fn collection(&self) -> Collection<Forum> {
        self.database_data.collection(CollectionName::Forums)
    }

    ///根据id查找论坛
    pub async fn find_by_id(&self, id: i64) -> mongodb::error::Result<Option<Forum>> {
        let filter = doc! {
            "_id": id,
        };
        let coll = self.collection();
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
