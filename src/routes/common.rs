use crate::http::controllers;
use actix_web::web::{self, ServiceConfig};

/// 加载普通路由
pub fn load_routes(cfg: &mut ServiceConfig) {
    cfg.service(controllers::hello_world)
        .route("/", web::get().to(|| async { "hello world".to_string() }));
}
