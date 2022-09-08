use crate::http::controllers::{self, api::captcha, api::session};
use actix_web::{web::ServiceConfig, Scope};

/// 加载api路由
pub fn load_routes(scope: Scope, cfg: &mut ServiceConfig) {
    let scope = scope
        .service(controllers::hello_world)
        .service(controllers::error_demo_fn)
        .service(session::init_session)
        .service(captcha::show);
    cfg.service(scope);
}
