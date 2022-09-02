use crate::routes;
use rocket::{Build, Rocket};

///rocket附加函数扩展
pub trait RocketExt {
    ///添加路由
    fn add_routes(self) -> Self;
    ///添加service
    fn add_services(self) -> Self;
}

impl RocketExt for Rocket<Build> {
    fn add_routes(self) -> Self {
        routes::load(self)
    }

    fn add_services(self) -> Self {
        //todo 待完善
        self
    }
}
