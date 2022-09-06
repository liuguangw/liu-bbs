mod api;
mod common;
use actix_web::web;

///配置路由
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    common::load_routes(cfg);
    let scope = web::scope("/api");
    api::load_routes(scope, cfg);
}
