use crate::{
    common::{CollectionName, DatabaseData},
    models::Session,
};
use mongodb::{
    bson::{doc, Document},
    Collection,
};
use std::{collections::HashMap, sync::Arc};

///session repository
pub struct SessionRepository {
    database_data: Arc<DatabaseData>,
}

impl SessionRepository {
    ///构造函数
    pub fn new(database_data: &Arc<DatabaseData>) -> Self {
        Self {
            database_data: database_data.clone(),
        }
    }
    fn collection(&self) -> Collection<Session> {
        self.database_data.collection(&CollectionName::Sessions)
    }

    ///保存会话到数据库
    pub async fn save_session(&self, session: &mut Session) -> mongodb::error::Result<()> {
        if session.id.is_empty() {
            self.insert_session(session).await
        } else {
            self.update_session(session).await
        }
    }

    async fn insert_session(&self, session: &mut Session) -> mongodb::error::Result<()> {
        session.set_random_id();
        let coll = self.collection();
        coll.insert_one(session, None).await?;
        Ok(())
    }

    fn serialize_data(data: &HashMap<String, String>) -> Document {
        let mut document = Document::new();
        for (k, v) in data {
            document.insert(k, v);
        }
        document
    }

    async fn update_session(&self, session: &Session) -> mongodb::error::Result<()> {
        let update_data = doc! {
            "$set":doc! {
            "user_id":session.user_id,
            "data":Self::serialize_data(&session.data),
            }
        };
        let filter = doc! {
            "_id":&session.id,
        };

        let coll = self.collection();
        coll.update_one(filter, update_data, None).await?;
        Ok(())
    }

    ///根据id查找会话
    pub async fn find_by_id(&self, id: &str) -> mongodb::error::Result<Option<Session>> {
        let filter = doc! {
            "_id":id,
        };
        let coll = self.collection();
        coll.find_one(filter, None).await
    }
}
