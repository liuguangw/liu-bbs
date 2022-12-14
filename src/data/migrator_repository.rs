use crate::common::{CollectionName, DatabaseData, Migration, MigrationLog};
use crate::migrations::{
    CreateCountersCollection, CreateDefaultForums, CreateForumGroupsCollection,
    CreateForumTreesCollection, CreateForumsCollection, CreateSessionsCollection,
    CreateTopicContentsCollection, CreateTopicRepliesCollection, CreateTopicsCollection,
    CreateUserEmailsCollection, CreateUsersCollection,
};
use futures_util::stream::TryStreamExt;
use mongodb::bson::doc;
use mongodb::options::FindOptions;
use mongodb::Collection;
use std::sync::Arc;

///migrator repository
pub struct MigratorRepository {
    database_data: Arc<DatabaseData>,
}

impl From<&Arc<DatabaseData>> for MigratorRepository {
    fn from(database_data: &Arc<DatabaseData>) -> Self {
        Self {
            database_data: database_data.clone(),
        }
    }
}

impl MigratorRepository {
    ///获取迁移记录集合对象
    fn migration_log_collection(&self) -> Collection<MigrationLog> {
        self.database_data.collection(CollectionName::Migrations)
    }
    ///插入一条迁移记录
    pub async fn insert_log(&self, migration_log: &MigrationLog) -> mongodb::error::Result<()> {
        let coll = self.migration_log_collection();
        coll.insert_one(migration_log, None).await?;
        Ok(())
    }
    ///删除一条迁移记录
    pub async fn remove_log(&self, id: i64) -> mongodb::error::Result<()> {
        let coll = self.migration_log_collection();
        coll.delete_one(
            doc! {
                "_id":id,
            },
            None,
        )
        .await?;
        Ok(())
    }

    ///获取所有的迁移记录
    pub async fn get_migration_logs(
        &self,
        sort_type: i32,
    ) -> mongodb::error::Result<Vec<MigrationLog>> {
        let coll = self.migration_log_collection();
        let find_options = FindOptions::builder()
            .sort(doc! { "_id": sort_type })
            .build();
        let cursor = coll.find(None, find_options).await?;
        let items = cursor.try_collect().await?;
        Ok(items)
    }
    ///构造迁移对象列表
    pub fn all_migrations(&self) -> Vec<Box<dyn Migration>> {
        vec![
            Box::new(CreateCountersCollection::new(&self.database_data)),
            Box::new(CreateUsersCollection::new(&self.database_data)),
            Box::new(CreateUserEmailsCollection::new(&self.database_data)),
            Box::new(CreateSessionsCollection::new(&self.database_data)),
            Box::new(CreateForumGroupsCollection::new(&self.database_data)),
            Box::new(CreateForumsCollection::new(&self.database_data)),
            Box::new(CreateForumTreesCollection::new(&self.database_data)),
            Box::new(CreateDefaultForums::new(&self.database_data)),
            Box::new(CreateTopicsCollection::new(&self.database_data)),
            Box::new(CreateTopicContentsCollection::new(&self.database_data)),
            Box::new(CreateTopicRepliesCollection::new(&self.database_data)),
        ]
    }
}
