use crate::http::controllers;
use actix_web::{web::ServiceConfig, Scope};

/// 加载api路由
pub fn load_routes(scope: Scope, cfg: &mut ServiceConfig) {
    let scope = scope
        .service(controllers::hello_world)
        .service(controllers::error_demo_fn);
    cfg.service(scope);
}
