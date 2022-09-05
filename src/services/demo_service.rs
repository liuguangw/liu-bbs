use crate::data::DemoRepository;
use std::sync::Arc;
/// demo service
pub struct DemoService {
    demo_repo: Arc<DemoRepository>,
}

impl DemoService {
    ///构造函数
    pub fn new(demo_repo: &Arc<DemoRepository>) -> Self {
        Self {
            demo_repo: demo_repo.clone(),
        }
    }
    ///hello demo
    pub async fn hello(&self) -> mongodb::error::Result<String> {
        let coll_names = self.demo_repo.list_collection_names().await?;
        let db_version = self.demo_repo.db_version().await?;
        Ok(format!(
            "db_version: {}, collections: {}",
            &db_version,
            coll_names.join(", ")
        ))
    }
}
