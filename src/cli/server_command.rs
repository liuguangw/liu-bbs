use super::app_command::AppCommand;
use crate::common::{AppConfig, DatabaseData, LaunchError, MigrationError};
use crate::data::{DemoRepository, MigratorRepository};
use crate::routes;
use crate::services::{DemoService, MigratorService};
use actix_web::{rt, web, App, HttpServer};
use clap::Args;
use std::sync::Arc;

/// 运行 `HTTP` 服务的命令
#[derive(Args)]
pub struct ServerCommand {
    ///IP address
    #[clap(short = 'H', long, value_parser)]
    host: Option<String>,
    ///listen port
    #[clap(short = 'P', long, value_parser)]
    port: Option<u16>,
    ///Path of configuration file
    #[clap(short = 'C', long = "conf", value_parser, default_value_t = String::from("./config.toml"), value_name = "FILE")]
    config_file_path: String,
}

impl ServerCommand {
    async fn do_migrate(&self, database_data: &Arc<DatabaseData>) -> Result<(), MigrationError> {
        let migrator_repo = MigratorRepository::new(database_data);
        let migrator_service = MigratorService::new(migrator_repo);
        let migrate_count = migrator_service.run_migrate(None).await?;
        if migrate_count > 0 {
            println!("Migrate count: {}", migrate_count);
        }
        Ok(())
    }
    /// 启动web服务
    async fn launch(&self) -> Result<(), LaunchError> {
        //加载配置
        let app_config = AppConfig::load(&self.config_file_path).await?;
        //连接数据库
        let database_data = DatabaseData::connect(&app_config.database).await?;
        let database_data = Arc::new(database_data);
        //执行数据迁移
        self.do_migrate(&database_data).await?;
        //run web server
        HttpServer::new(move || {
            App::new()
                .configure(routes::configure_routes)
                .configure(|cfg| configure_data(cfg, &database_data))
        })
        .bind((app_config.server.host.as_str(), app_config.server.port))?
        .run()
        .await?;
        Ok(())
    }
}

impl AppCommand for ServerCommand {
    /// 运行 http 服务
    fn execute(&self) {
        //println!("host={} port={}", &self.host, self.port);
        if let Err(err) = rt::System::new().block_on(self.launch()) {
            panic!("{}", err)
        }
    }
}

///配置共享数据
fn configure_data(cfg: &mut web::ServiceConfig, database_data: &Arc<DatabaseData>) {
    let demo_repo = Arc::new(DemoRepository::new(database_data));
    let demo_service = DemoService::new(&demo_repo);
    cfg.app_data(web::Data::new(demo_service));
}
