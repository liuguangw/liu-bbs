use crate::common::{CollectionName, DatabaseData, Migration, MigrationError};
use async_trait::async_trait;
use mongodb::bson::doc;
use mongodb::bson::document::Document;
use mongodb::options::IndexOptions;
use mongodb::{Collection, IndexModel};
use std::sync::Arc;

///创建用户集合
///
/// name: `2022_09_06_000000_create_users_collection`
pub struct CreateUsersCollection {
    database_data: Arc<DatabaseData>,
}
impl CreateUsersCollection {
    ///构造函数
    pub fn new(database_data: &Arc<DatabaseData>) -> Self {
        Self {
            database_data: database_data.clone(),
        }
    }
    ///获取集合对象
    fn collection(&self) -> Collection<Document> {
        self.database_data
            .collection::<Document>(&CollectionName::Users)
    }
}

#[async_trait]
impl Migration for CreateUsersCollection {
    fn name(&self) -> &str {
        "2022_09_06_000000_create_users_collection"
    }

    async fn up(&self) -> Result<(), MigrationError> {
        let coll = self.collection();
        //创建索引
        let index_models = vec![
            IndexModel::builder()
                .keys(doc! {
                    "username": 1,
                })
                .options(
                    IndexOptions::builder()
                        .name("username_uni".to_string())
                        .unique(true)
                        .build(),
                )
                .build(),
            IndexModel::builder()
                .keys(doc! {
                    "nickname": 1,
                })
                .options(
                    IndexOptions::builder()
                        .name("nickname_uni".to_string())
                        .unique(true)
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
