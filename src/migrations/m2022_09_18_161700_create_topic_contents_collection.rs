use crate::common::{CollectionName, DatabaseData, Migration, MigrationError};
use async_trait::async_trait;
use mongodb::bson::document::Document;
use mongodb::Collection;
use std::sync::Arc;

///创建帖子内容集合
///
/// name: `2022_09_18_161700_create_topic_contents_collection`
pub struct CreateTopicContentsCollection {
    database_data: Arc<DatabaseData>,
}
impl CreateTopicContentsCollection {
    ///构造函数
    pub fn new(database_data: &Arc<DatabaseData>) -> Self {
        Self {
            database_data: database_data.clone(),
        }
    }
    ///获取集合完整名称
    fn collection_full_name(&self) -> String {
        self.database_data
            .collection_full_name(CollectionName::TopicContents)
    }
    ///获取集合对象
    fn collection(&self) -> Collection<Document> {
        self.database_data.collection(CollectionName::TopicContents)
    }
}

#[async_trait]
impl Migration for CreateTopicContentsCollection {
    fn name(&self) -> &str {
        "2022_09_18_161700_create_topic_contents_collection"
    }

    async fn up(&self) -> Result<(), MigrationError> {
        let coll_name = self.collection_full_name();
        self.database_data
            .database
            .create_collection(&coll_name, None)
            .await?;
        Ok(())
    }

    async fn down(&self) -> Result<(), MigrationError> {
        let coll = self.collection();
        coll.drop(None).await?;
        Ok(())
    }
}
