use crate::common::{DatabaseData, Migration, MigrationError};
use async_trait::async_trait;
use mongodb::bson::document::Document;
use mongodb::Collection;
use std::sync::Arc;

///创建计数器集合
pub struct CreateCountersCollection {
    database_data: Arc<DatabaseData>,
}
impl CreateCountersCollection {
    pub fn new(database_data: &Arc<DatabaseData>) -> Self {
        Self {
            database_data: database_data.clone(),
        }
    }
    fn collection_name(&self) -> String {
        self.database_data.collection_full_name("counters")
    }
}

#[async_trait]
impl Migration for CreateCountersCollection {
    fn name(&self) -> &str {
        "2022_08_19_000000_create_counters_collection"
    }

    async fn up(&self) -> Result<(), MigrationError> {
        let coll_name = self.collection_name();
        self.database_data
            .database
            .create_collection(&coll_name, None)
            .await?;
        Ok(())
    }

    async fn down(&self) -> Result<(), MigrationError> {
        let coll_name = self.collection_name();
        let coll: Collection<Document> = self.database_data.database.collection(&coll_name);
        coll.drop(None).await?;
        Ok(())
    }
}
