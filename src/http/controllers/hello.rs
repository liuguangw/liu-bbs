use crate::common::{DatabaseData, ResponseResult};
use rocket::State;

/// 输出hello world
#[rocket::get("/hello")]
pub async fn hello(database_data: &State<DatabaseData>) -> ResponseResult<String> {
    let version = database_data.db_version().await?;
    Ok(format!("Hello, world, database version: {}", version).into())
}
