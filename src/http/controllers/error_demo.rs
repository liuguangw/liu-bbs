use crate::common::{ApiError, ResponseResult};
use actix_web::get;

/// 简单的通用错误
#[get("/error")]
pub async fn error_demo_fn() -> ResponseResult<()> {
    Err(ApiError::Common("this is a common error".to_string()))
}
