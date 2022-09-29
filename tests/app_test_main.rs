use actix_web::{test, web, App};
use liu_bbs::{
    cli::app_factory,
    common::{AppConfig, DatabaseData, MigrationError},
    data::MigratorRepository,
    services::{MigratorService, Provider},
};
use std::sync::Arc;

#[actix_web::test]
async fn test_main() {
    //加载配置
    let app_config = AppConfig::load("./config.toml").await;
    assert!(app_config.is_ok(), "load app config ok");
    let app_config = app_config.unwrap();
    //连接数据库
    let database_data = DatabaseData::connect(&app_config.database).await;
    assert!(database_data.is_ok(), "connect database ok");
    let database_data = database_data.unwrap();
    let database_data = Arc::new(database_data);
    //执行数据迁移
    let migrate_result = do_migrate(&database_data).await;
    assert!(migrate_result.is_ok(), "migrate ok");
    //初始化service provider
    let service_provider = web::Data::new(Provider::new(&database_data));
    //
    let app = test::init_service(app_factory(App::new(), &service_provider)).await;
    let req = test::TestRequest::post()
        .uri("/api/session/init")
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success(), "request ok");
}

async fn do_migrate(database_data: &Arc<DatabaseData>) -> Result<u32, MigrationError> {
    let migrator_repo = MigratorRepository::new(database_data);
    let migrator_service = MigratorService::new(migrator_repo);
    migrator_service.run_migrate(None).await
}
