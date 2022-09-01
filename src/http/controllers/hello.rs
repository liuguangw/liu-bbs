/// 输出hello world
#[rocket::get("/hello")]
pub fn hello() -> &'static str {
    "Hello, world!"
}
