///api控制器
pub mod api;
mod error_demo;
mod hello;
#[cfg(feature = "swagger-ui")]
mod openapi;
pub use error_demo::error_demo_fn;
pub use hello::hello as hello_world;
#[cfg(feature = "swagger-ui")]
pub use openapi::yml_file as openapi_yml;
