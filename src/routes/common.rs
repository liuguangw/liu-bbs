use crate::http::controllers;
use actix_web::web::{self, ServiceConfig};

/// 加载普通路由
pub fn load_routes(cfg: &mut ServiceConfig) {
    //swagger-ui 路由
    #[cfg(feature = "swagger-ui")]
    {
        use actix_files::Files;
        cfg.service(
            Files::new("/swagger-ui", "./public/swagger-ui")
                .index_file("index.html")
                .redirect_to_slash_directory(),
        )
        .service(controllers::openapi_yml);
    }
    cfg.service(controllers::hello_world)
        .route("/", web::get().to(|| async { "hello world".to_string() }));
}
