use crate::http::controllers;
use rocket::{routes, Build, Rocket};

/// 加载api路由
pub fn load_routes(b: Rocket<Build>) -> Rocket<Build> {
    b.mount("/api", routes![controllers::hello_world])
}
