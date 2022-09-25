use crate::http::controllers;
use actix_web::web::ServiceConfig;

/// 加载普通路由
pub fn load_routes(cfg: &mut ServiceConfig) {
    //openapi-doc 路由
    #[cfg(feature = "openapi-doc")]
    {
        use actix_files::Files;
        cfg.service(
            Files::new("/openapi-doc", "./public/openapi_doc")
                .index_file("index.html")
                .redirect_to_slash_directory(),
        )
        .service(
            Files::new("/openapi", "./public/openapi")
                .index_file("index.html")
                .redirect_to_slash_directory(),
        );
    }
    cfg.service(controllers::hello);
}
