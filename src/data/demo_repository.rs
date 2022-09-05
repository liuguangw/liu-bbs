use crate::common::DatabaseData;
use std::sync::Arc;
///demo repository
pub struct DemoRepository {
    database_data: Arc<DatabaseData>,
}

impl DemoRepository {
    ///构造函数
    pub fn new(database_data: &Arc<DatabaseData>) -> Self {
        Self {
            database_data: database_data.clone(),
        }
    }
    ///demo
    pub async fn list_collection_names(&self) -> mongodb::error::Result<Vec<String>> {
        self.database_data
            .database
            .list_collection_names(None)
            .await
    }
    ///demo
    pub async fn db_version(&self) -> Result<String, mongodb::error::Error> {
        self.database_data.db_version().await
    }
}
