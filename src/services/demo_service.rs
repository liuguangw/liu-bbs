use crate::{common::DatabaseResult, data::DemoRepository};
/// demo service
pub struct DemoService {
    demo_repo: DemoRepository,
}

impl DemoService {
    ///构造函数
    pub fn new(demo_repo: DemoRepository) -> Self {
        Self { demo_repo }
    }
    ///hello demo
    pub async fn hello(&self) -> DatabaseResult<String> {
        let coll_names = self.demo_repo.list_collection_names().await?;
        let db_version = self.demo_repo.db_version().await?;
        Ok(format!(
            "db_version: {}, collections: {}",
            &db_version,
            coll_names.join(", ")
        ))
    }
}
