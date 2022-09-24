use actix_web::get;

/// 跳转到swagger-ui页面
#[cfg(feature = "swagger-ui")]
#[get("/")]
pub async fn hello() -> actix_web::HttpResponse {
    use actix_web::http::header;
    actix_web::HttpResponse::Found()
        .insert_header((header::LOCATION, "/swagger-ui/"))
        .finish()
}

/// 输出hello world
#[cfg(not(feature = "swagger-ui"))]
#[get("/")]
pub async fn hello() -> &'static str {
    "hello world"
}
