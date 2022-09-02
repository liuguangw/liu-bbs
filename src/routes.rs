//! 路由
mod api;
mod common;

use rocket::{Build, Rocket};

///加载路由
pub fn load(b: Rocket<Build>) -> Rocket<Build> {
    let r = common::load_routes(b);
    api::load_routes(r)
}
