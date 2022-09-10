use crate::{
    common::{CollectionName, DatabaseData},
    models::User,
};
use mongodb::{bson::doc, Collection};
use std::sync::Arc;

///user repository
pub struct UserRepository {
    database_data: Arc<DatabaseData>,
}

impl UserRepository {
    ///构造函数
    pub fn new(database_data: &Arc<DatabaseData>) -> Self {
        Self {
            database_data: database_data.clone(),
        }
    }

    fn collection(&self) -> Collection<User> {
        self.database_data.collection(CollectionName::Users)
    }

    ///insert new user
    pub async fn insert_user(&self, user: &User) -> mongodb::error::Result<()> {
        let coll = self.collection();
        coll.insert_one(user, None).await?;
        Ok(())
    }

    ///根据id查找用户
    pub async fn find_by_id(&self, id: i64) -> mongodb::error::Result<Option<User>> {
        let filter = doc! {
            "_id": id,
        };
        let coll = self.collection();
        coll.find_one(filter, None).await
    }

    ///根据用户名查找用户
    pub async fn find_by_username(&self, username: &str) -> mongodb::error::Result<Option<User>> {
        let filter = doc! {
            "username": username,
        };
        let coll = self.collection();
        coll.find_one(filter, None).await
    }

    ///根据昵称查找用户
    pub async fn find_by_nickname(&self, nickname: &str) -> mongodb::error::Result<Option<User>> {
        let filter = doc! {
            "nickname": nickname,
        };
        let coll = self.collection();
        coll.find_one(filter, None).await
    }
}
