use crate::common::{CollectionName, DatabaseData, Migration, MigrationError};
use async_trait::async_trait;
use mongodb::bson::doc;
use mongodb::bson::document::Document;
use mongodb::options::IndexOptions;
use mongodb::{Collection, IndexModel};
use std::sync::Arc;

///创建回帖集合
///
/// name: `2022_09_18_164400_create_topic_replies_collection`
pub struct CreateTopicRepliesCollection {
    database_data: Arc<DatabaseData>,
}
impl CreateTopicRepliesCollection {
    ///构造函数
    pub fn new(database_data: &Arc<DatabaseData>) -> Self {
        Self {
            database_data: database_data.clone(),
        }
    }
    ///获取集合对象
    fn collection(&self) -> Collection<Document> {
        self.database_data.collection(CollectionName::TopicReplies)
    }
}

#[async_trait]
impl Migration for CreateTopicRepliesCollection {
    fn name(&self) -> &str {
        "2022_09_18_164400_create_topic_replies_collection"
    }

    async fn up(&self) -> Result<(), MigrationError> {
        let coll = self.collection();
        //创建索引
        let index_models = vec![
            IndexModel::builder()
                .keys(doc! {
                    "topic_id": 1,
                    "floor_number": 1,
                })
                .options(
                    IndexOptions::builder()
                        .name("topic_floor_uni".to_string())
                        .unique(true)
                        .build(),
                )
                .build(),
            IndexModel::builder()
                .keys(doc! {
                    "author_user_id": 1,
                })
                .options(
                    IndexOptions::builder()
                        .name("author_index".to_string())
                        .build(),
                )
                .build(),
        ];
        coll.create_indexes(index_models, None).await?;
        Ok(())
    }

    async fn down(&self) -> Result<(), MigrationError> {
        let coll = self.collection();
        coll.drop(None).await?;
        Ok(())
    }
}
