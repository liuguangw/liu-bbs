use crate::common::ResponseResult;
use crate::services::DemoService;
use rocket::State;

/// 输出hello world
#[rocket::get("/hello")]
pub async fn hello(demo_service: &State<DemoService>) -> ResponseResult<String> {
    let str = demo_service.hello().await?;
    Ok(str.into())
}
