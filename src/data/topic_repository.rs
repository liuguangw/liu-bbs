use crate::{
    common::{CollectionName, DatabaseData},
    models::Topic,
};
use futures_util::TryStreamExt;
use mongodb::{bson::doc, options::FindOptions, Collection};
use std::sync::Arc;

///topic repository
pub struct TopicRepository {
    database_data: Arc<DatabaseData>,
}

impl From<&Arc<DatabaseData>> for TopicRepository {
    fn from(database_data: &Arc<DatabaseData>) -> Self {
        Self {
            database_data: database_data.clone(),
        }
    }
}

impl TopicRepository {
    fn collection(&self) -> Collection<Topic> {
        self.database_data.collection(CollectionName::Topics)
    }

    ///insert new topic
    pub async fn insert_topic(&self, item: &Topic) -> mongodb::error::Result<()> {
        let coll = self.collection();
        coll.insert_one(item, None).await?;
        Ok(())
    }

    ///根据id查找topic
    pub async fn find_by_id(&self, id: i64) -> mongodb::error::Result<Option<Topic>> {
        let filter = doc! {
            "_id": id,
        };
        let coll = self.collection();
        coll.find_one(filter, None).await
    }

    ///获取论坛帖子列表
    pub async fn get_forum_topic_list(
        &self,
        forum_id: i64,
        sort_type: u8,
        offset: u64,
        limit: i64,
    ) -> mongodb::error::Result<Vec<Topic>> {
        let filter = doc! {
            "forum_id" : forum_id,
            "is_publish" : true,
            "is_delete" : false
        };
        let sort_opt = if sort_type == 1 {
            doc! {
                "updated_at" : -1
            }
        } else {
            doc! {
                "publish_at" : -1,
                "_id" : -1
            }
        };
        let find_options = FindOptions::builder()
            .sort(sort_opt)
            .skip(offset)
            .limit(limit)
            .build();
        let coll = self.collection();
        let cursor = coll.find(filter, find_options).await?;
        cursor.try_collect().await
    }

    ///计算帖子总数
    pub async fn get_forum_topic_count(&self, forum_id: i64) -> mongodb::error::Result<u64> {
        let filter = doc! {
            "forum_id" : forum_id,
            "is_publish" : true,
            "is_delete" : false
        };
        let coll = self.collection();
        coll.count_documents(filter, None).await
    }
}
