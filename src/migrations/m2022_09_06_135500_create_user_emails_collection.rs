use crate::common::{CollectionName, DatabaseData, Migration, MigrationError};
use async_trait::async_trait;
use mongodb::bson::doc;
use mongodb::bson::document::Document;
use mongodb::options::IndexOptions;
use mongodb::{Collection, IndexModel};
use std::sync::Arc;

///创建用户邮箱绑定记录集合
///
/// name: `2022_09_06_000000_create_user_emails_collection`
pub struct CreateUserEmailsCollection {
    database_data: Arc<DatabaseData>,
}
impl CreateUserEmailsCollection {
    ///构造函数
    pub fn new(database_data: &Arc<DatabaseData>) -> Self {
        Self {
            database_data: database_data.clone(),
        }
    }
    ///获取集合对象
    fn collection(&self) -> Collection<Document> {
        self.database_data.collection(CollectionName::UserEmails)
    }
}

#[async_trait]
impl Migration for CreateUserEmailsCollection {
    fn name(&self) -> &str {
        "2022_09_06_000000_create_user_emails_collection"
    }

    async fn up(&self) -> Result<(), MigrationError> {
        let coll = self.collection();
        //创建索引
        let index_models = vec![
            IndexModel::builder()
                .keys(doc! {
                    "email": 1,
                })
                .options(
                    IndexOptions::builder()
                        .name("email_uni".to_string())
                        .unique(true)
                        .build(),
                )
                .build(),
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
