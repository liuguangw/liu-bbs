use crate::common::{CollectionName, DatabaseData, Migration, MigrationError};
use async_trait::async_trait;
use mongodb::bson::doc;
use mongodb::bson::document::Document;
use mongodb::options::IndexOptions;
use mongodb::{Collection, IndexModel};
use std::sync::Arc;
use std::time::Duration;

///创建会话集合
///
/// name: `2022_09_06_230900_create_sessions_collection`
pub struct CreateSessionsCollection {
    database_data: Arc<DatabaseData>,
}
impl CreateSessionsCollection {
    ///构造函数
    pub fn new(database_data: &Arc<DatabaseData>) -> Self {
        Self {
            database_data: database_data.clone(),
        }
    }
    ///获取集合对象
    fn collection(&self) -> Collection<Document> {
        self.database_data.collection(CollectionName::Sessions)
    }
}

#[async_trait]
impl Migration for CreateSessionsCollection {
    fn name(&self) -> &str {
        "2022_09_06_230900_create_sessions_collection"
    }

    async fn up(&self) -> Result<(), MigrationError> {
        let coll = self.collection();
        //创建索引
        let index_models = vec![
            IndexModel::builder()
                .keys(doc! {
                    "user_id": 1,
                })
                .options(
                    IndexOptions::builder()
                        .name("user_id_index".to_string())
                        .build(),
                )
                .build(),
            IndexModel::builder()
                .keys(doc! {
                    "expired_at": 1,
                })
                .options(
                    IndexOptions::builder()
                        .name("expired_at_ttl".to_string())
                        .expire_after(Duration::from_secs(1))
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
