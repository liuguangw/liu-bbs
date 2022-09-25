use actix_web::get;

/// 跳转到openapi文档页面
#[cfg(feature = "openapi-doc")]
#[get("/")]
pub async fn hello() -> actix_web::HttpResponse {
    use actix_web::http::header;
    actix_web::HttpResponse::Found()
        .insert_header((header::LOCATION, "/openapi-doc/"))
        .finish()
}

/// 输出hello world
#[cfg(not(feature = "openapi-doc"))]
#[get("/")]
pub async fn hello() -> &'static str {
    "hello world"
}
