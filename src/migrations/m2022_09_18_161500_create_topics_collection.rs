use crate::common::{CollectionName, DatabaseData, Migration, MigrationError};
use async_trait::async_trait;
use mongodb::bson::doc;
use mongodb::bson::document::Document;
use mongodb::options::IndexOptions;
use mongodb::{Collection, IndexModel};
use std::sync::Arc;

///创建帖子集合
///
/// name: `2022_09_18_161500_create_topics_collection`
pub struct CreateTopicsCollection {
    database_data: Arc<DatabaseData>,
}
impl CreateTopicsCollection {
    ///构造函数
    pub fn new(database_data: &Arc<DatabaseData>) -> Self {
        Self {
            database_data: database_data.clone(),
        }
    }
    ///获取集合对象
    fn collection(&self) -> Collection<Document> {
        self.database_data.collection(CollectionName::Topics)
    }
}

#[async_trait]
impl Migration for CreateTopicsCollection {
    fn name(&self) -> &str {
        "2022_09_18_161500_create_topics_collection"
    }

    async fn up(&self) -> Result<(), MigrationError> {
        let coll = self.collection();
        //创建索引
        let index_models = vec![
            IndexModel::builder()
                .keys(doc! {
                    "author_user_id": 1,
                })
                .options(
                    IndexOptions::builder()
                        .name("author_user_id_index".to_string())
                        .build(),
                )
                .build(),
            IndexModel::builder()
                .keys(doc! {
                    "forum_id": 1,
                })
                .options(
                    IndexOptions::builder()
                        .name("forum_id_index".to_string())
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
