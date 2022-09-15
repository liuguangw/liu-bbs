use crate::common::{CollectionName, DatabaseData, Migration, MigrationError};
use async_trait::async_trait;
use mongodb::bson::doc;
use mongodb::bson::document::Document;
use mongodb::options::IndexOptions;
use mongodb::{Collection, IndexModel};
use std::sync::Arc;

///创建论坛集合
///
/// name: `2022_09_15_164800_create_forums_collection`
pub struct CreateForumsCollection {
    database_data: Arc<DatabaseData>,
}
impl CreateForumsCollection {
    ///构造函数
    pub fn new(database_data: &Arc<DatabaseData>) -> Self {
        Self {
            database_data: database_data.clone(),
        }
    }
    ///获取集合对象
    fn collection(&self) -> Collection<Document> {
        self.database_data.collection(CollectionName::Forums)
    }
}

#[async_trait]
impl Migration for CreateForumsCollection {
    fn name(&self) -> &str {
        "2022_09_15_164800_create_forums_collection"
    }

    async fn up(&self) -> Result<(), MigrationError> {
        let coll = self.collection();
        //创建索引
        let index_models = vec![
            IndexModel::builder()
                .keys(doc! {
                    "forum_group_id": 1,
                })
                .options(
                    IndexOptions::builder()
                        .name("forum_group_id_index".to_string())
                        .build(),
                )
                .build(),
            IndexModel::builder()
                .keys(doc! {
                    "order_id": 1,
                })
                .options(
                    IndexOptions::builder()
                        .name("order_id_index".to_string())
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
