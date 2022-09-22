use crate::{
    common::{CollectionName, DatabaseData},
    models::Topic,
};
use mongodb::{bson::doc, Collection};
use std::sync::Arc;

///topic repository
pub struct TopicRepository {
    database_data: Arc<DatabaseData>,
}

impl TopicRepository {
    ///构造函数
    pub fn new(database_data: &Arc<DatabaseData>) -> Self {
        Self {
            database_data: database_data.clone(),
        }
    }

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
}
