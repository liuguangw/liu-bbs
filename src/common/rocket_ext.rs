use crate::common::DatabaseData;
use crate::data::DemoRepository;
use crate::routes;
use crate::services::DemoService;
use rocket::{Build, Rocket};
use std::sync::Arc;

///rocket附加函数扩展
pub trait RocketExt {
    ///添加路由
    fn attach_routes(self) -> Self;
    ///添加service
    fn attach_services(self, database_data: Arc<DatabaseData>) -> Self;
}

impl RocketExt for Rocket<Build> {
    fn attach_routes(self) -> Self {
        routes::load(self)
    }

    fn attach_services(self, database_data: Arc<DatabaseData>) -> Self {
        let demo_repo = Arc::new(DemoRepository::new(&database_data));
        let demo_service = DemoService::new(&demo_repo);
        self.manage(demo_service)
    }
}
