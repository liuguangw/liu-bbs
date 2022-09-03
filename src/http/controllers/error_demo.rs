use crate::common::{ApiError, ResponseResult};

/// 简单的通用错误
#[rocket::get("/error")]
pub async fn error_demo_fn() -> ResponseResult<()> {
    Err(ApiError::Common("this is a common error".to_string()))
}
