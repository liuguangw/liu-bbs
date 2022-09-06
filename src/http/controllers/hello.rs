use crate::common::ResponseResult;
use crate::services::DemoService;
use actix_web::{get, web};

/// 输出hello world
#[get("/hello")]
pub async fn hello(demo_service: web::Data<DemoService>) -> ResponseResult<String> {
    let str = demo_service.hello().await?;
    Ok(str.into())
}
