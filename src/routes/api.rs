use crate::{
    common::ApiError,
    http::controllers::{self, api::auth, api::captcha, api::session},
};
use actix_web::{
    web::{JsonConfig, ServiceConfig},
    Scope,
};

/// 加载api路由
pub fn load_routes(scope: Scope, cfg: &mut ServiceConfig) {
    // custom `Json` extractor configuration for api scope
    let json_cfg = JsonConfig::default()
        .error_handler(|err, _req| ApiError::BadRequest(err.to_string()).into());
    let scope = scope
        .app_data(json_cfg)
        .service(controllers::hello_world)
        .service(controllers::error_demo_fn)
        .service(session::init_session)
        .service(captcha::show)
        .service(auth::register)
        .service(auth::login);
    cfg.service(scope);
}
