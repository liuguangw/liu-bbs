use crate::http::controllers::{api::auth, api::captcha, api::session, api::topic, api::user};
use actix_web::{web::ServiceConfig, Scope};

/// 加载api路由
pub fn load_routes(scope: Scope, cfg: &mut ServiceConfig) {
    let scope = scope
        .service(session::init_session)
        .service(captcha::show)
        .service(auth::register)
        .service(auth::login)
        .service(user::info)
        .service(topic::store);
    cfg.service(scope);
}
