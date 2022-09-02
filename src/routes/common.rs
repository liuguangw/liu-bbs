use crate::http::controllers;
use rocket::{routes, Build, Rocket};
/// 加载普通路由
pub fn load_routes(b: Rocket<Build>) -> Rocket<Build> {
    b.mount("/", routes![controllers::hello_world])
}
