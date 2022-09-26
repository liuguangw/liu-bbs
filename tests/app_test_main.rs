use actix_web::{test, web, App};
use liu_bbs::{
    cli::app_factory,
    common::{AppConfig, DatabaseData},
    services::CaptchaService,
};
use std::sync::Arc;

//use liu_bbs::cli::app_factory;
#[actix_web::test]
async fn test_main() {
    //加载配置
    let app_config = AppConfig::load("./config.toml").await;
    assert!(app_config.is_ok());
    let app_config = app_config.unwrap();
    //连接数据库
    let database_data = DatabaseData::connect(&app_config.database).await;
    assert!(database_data.is_ok());
    let database_data = database_data.unwrap();
    let database_data = Arc::new(database_data);
    //captcha service(字体只加载一次)
    let captcha_service = web::Data::new(CaptchaService::default());
    //
    let app = test::init_service(app_factory(App::new(), &database_data, &captcha_service)).await;
    let req = test::TestRequest::post()
        .uri("/api/session/init")
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success(), "request ok");
}
