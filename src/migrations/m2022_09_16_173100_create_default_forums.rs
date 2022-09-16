use crate::common::{CollectionName, CounterKey, DatabaseData, Migration, MigrationError};
use crate::models::{Forum, ForumGroup, ForumTree};
use async_trait::async_trait;
use mongodb::bson::{doc, Document};
use mongodb::options::FindOneAndUpdateOptions;
use mongodb::Collection;
use std::sync::Arc;

///创建默认的论坛集合
///
/// name: `2022_09_16_173100_create_default_forums`
pub struct CreateDefaultForums {
    database_data: Arc<DatabaseData>,
}
impl CreateDefaultForums {
    ///构造函数
    pub fn new(database_data: &Arc<DatabaseData>) -> Self {
        Self {
            database_data: database_data.clone(),
        }
    }
    ///获取论坛分区集合对象
    fn group_collection(&self) -> Collection<ForumGroup> {
        self.database_data.collection(CollectionName::ForumGroups)
    }
    ///获取论坛集合对象
    fn forum_collection(&self) -> Collection<Forum> {
        self.database_data.collection(CollectionName::Forums)
    }
    ///获取论坛树结构集合对象
    fn tree_collection(&self) -> Collection<ForumTree> {
        self.database_data.collection(CollectionName::ForumTrees)
    }
    ///获取计数器集合对象
    fn counter_collection(&self) -> Collection<Document> {
        self.database_data.collection(CollectionName::Counters)
    }

    #[inline]
    fn is_debug() -> bool {
        cfg!(debug_assertions)
    }

    fn default_forums() -> Vec<Forum> {
        let default_description = "初次运行时,系统创建的论坛";
        let mut forums = vec![Forum {
            id: 1,
            forum_group_id: 1,
            is_root: true,
            name: String::from("默认论坛"),
            description: String::from(default_description),
            ..Default::default()
        }];
        if !Self::is_debug() {
            return forums;
        }
        let mut extra_forums = vec![
            Forum {
                id: 2,
                forum_group_id: 1,
                is_root: true,
                name: String::from("默认论坛a"),
                description: String::from(default_description),
                ..Default::default()
            },
            Forum {
                id: 3,
                forum_group_id: 1,
                is_root: false,
                name: String::from("默认论坛b"),
                description: String::from(default_description),
                ..Default::default()
            },
            Forum {
                id: 4,
                forum_group_id: 1,
                is_root: false,
                name: String::from("默认论坛c"),
                description: String::from(default_description),
                ..Default::default()
            },
            Forum {
                id: 5,
                forum_group_id: 1,
                is_root: false,
                name: String::from("默认论坛d"),
                description: String::from(default_description),
                ..Default::default()
            },
        ];
        forums.append(&mut extra_forums);
        forums
    }
}

#[async_trait]
impl Migration for CreateDefaultForums {
    fn name(&self) -> &str {
        "2022_09_16_173100_create_default_forums"
    }

    async fn up(&self) -> Result<(), MigrationError> {
        //创建默认分区
        let default_group = ForumGroup {
            id: 1,
            name: String::from("默认分区"),
            description: String::from("初次运行时,系统创建的分区"),
            ..Default::default()
        };
        let group_coll = self.group_collection();
        group_coll.insert_one(&default_group, None).await?;
        //创建论坛
        let forums = Self::default_forums();
        let forum_coll = self.forum_collection();
        forum_coll.insert_many(&forums, None).await?;
        //创建论坛树
        if Self::is_debug() {
            let forum_trees = vec![
                ForumTree {
                    forum_id: 2,
                    child_forum_id: 3,
                    path_deep: 1,
                },
                ForumTree {
                    forum_id: 2,
                    child_forum_id: 4,
                    path_deep: 1,
                },
                ForumTree {
                    forum_id: 2,
                    child_forum_id: 5,
                    path_deep: 2,
                },
                ForumTree {
                    forum_id: 4,
                    child_forum_id: 5,
                    path_deep: 1,
                },
            ];
            let tree_coll = self.tree_collection();
            tree_coll.insert_many(&forum_trees, None).await?;
        }
        //计数器incr
        let counter_opt = FindOneAndUpdateOptions::builder()
            //.return_document(ReturnDocument::After)
            .upsert(true)
            .build();
        let counter_coll = self.counter_collection();
        counter_coll
            .find_one_and_update(
                doc! {
                    "_id":CounterKey::LastForumGroupId.as_str(),
                },
                doc! {
                    "$inc": doc! {
                        "value": 1,
                    }
                },
                counter_opt.clone(),
            )
            .await?;
        counter_coll
            .find_one_and_update(
                doc! {
                    "_id":CounterKey::LastForumId.as_str(),
                },
                doc! {
                    "$inc": doc! {
                        "value": forums.len() as i32,
                    }
                },
                counter_opt,
            )
            .await?;
        Ok(())
    }

    async fn down(&self) -> Result<(), MigrationError> {
        let group_coll = self.group_collection();
        group_coll.delete_many(doc! {}, None).await?;
        //
        let forum_coll = self.group_collection();
        forum_coll.delete_many(doc! {}, None).await?;
        //
        let tree_coll = self.tree_collection();
        tree_coll.delete_many(doc! {}, None).await?;
        //
        let counter_coll = self.counter_collection();
        counter_coll
            .delete_one(
                doc! {
                    "_id": CounterKey::LastForumGroupId.as_str(),
                },
                None,
            )
            .await?;
        counter_coll
            .delete_one(
                doc! {
                    "_id": CounterKey::LastForumId.as_str(),
                },
                None,
            )
            .await?;
        Ok(())
    }
}
