use crate::common::DatabaseData;
use rocket::State;

/// 输出hello world
#[rocket::get("/hello")]
pub async fn hello(database_data: &State<DatabaseData>) -> String {
    let version = match database_data.db_version().await {
        Ok(v) => v,
        Err(err) => format!("{:?}", err),
    };
    format!("Hello, world!\ndatabase version: {}", version)
}
