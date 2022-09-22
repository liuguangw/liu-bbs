use crate::{
    common::{CollectionName, CounterKey, DatabaseData},
    models::Counter,
};
use mongodb::{
    bson::doc,
    options::{FindOneAndUpdateOptions, ReturnDocument},
    Collection,
};
use std::sync::Arc;

///counter repository
pub struct CounterRepository {
    database_data: Arc<DatabaseData>,
}

impl CounterRepository {
    ///构造函数
    pub fn new(database_data: &Arc<DatabaseData>) -> Self {
        Self {
            database_data: database_data.clone(),
        }
    }

    fn collection(&self) -> Collection<Counter> {
        self.database_data.collection(CollectionName::Counters)
    }

    ///增加计数器的值
    pub async fn increment(&self, key: CounterKey) -> mongodb::error::Result<i64> {
        let update_data = doc! {
            "$inc": doc! {
                "value": 1i64,
            }
        };
        let filter = doc! {
            "_id": key.as_str(),
        };
        let opt = FindOneAndUpdateOptions::builder()
            .return_document(ReturnDocument::After)
            .upsert(true)
            .build();
        let coll = self.collection();
        let item = coll.find_one_and_update(filter, update_data, opt).await?;
        Ok(item.unwrap().value)
    }
}
