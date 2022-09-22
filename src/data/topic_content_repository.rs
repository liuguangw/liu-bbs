use crate::{
    common::{CollectionName, DatabaseData},
    models::TopicContent,
};
use mongodb::{bson::doc, Collection};
use std::sync::Arc;

///topic content repository
pub struct TopicContentRepository {
    database_data: Arc<DatabaseData>,
}

impl TopicContentRepository {
    ///构造函数
    pub fn new(database_data: &Arc<DatabaseData>) -> Self {
        Self {
            database_data: database_data.clone(),
        }
    }

    fn collection(&self) -> Collection<TopicContent> {
        self.database_data.collection(CollectionName::TopicContents)
    }

    ///insert new topic content
    pub async fn insert_topic(&self, item: &TopicContent) -> mongodb::error::Result<()> {
        let coll = self.collection();
        coll.insert_one(item, None).await?;
        Ok(())
    }

    ///根据id查找topic content
    pub async fn find_by_id(&self, id: i64) -> mongodb::error::Result<Option<TopicContent>> {
        let filter = doc! {
            "_id": id,
        };
        let coll = self.collection();
        coll.find_one(filter, None).await
    }
}
