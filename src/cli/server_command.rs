use super::commands::Command;
use crate::common::{AppConfig, DatabaseData, LaunchError, MigrationError};
use crate::routes;
use crate::services::{MigratorService, Provider};
use actix_web::dev::{ServiceFactory, ServiceRequest};
use actix_web::{rt, web, App, HttpServer};
use clap::Args;
use std::sync::Arc;

/// 运行 `HTTP` 服务的命令
#[derive(Args)]
pub struct ServerCommand {
    ///IP address
    #[arg(short = 'H', long)]
    host: Option<String>,
    ///listen port
    #[arg(short = 'P', long)]
    port: Option<u16>,
    ///Path of configuration file
    #[arg(short = 'C', long = "conf", default_value_t = String::from("./config.toml"), value_name = "FILE")]
    config_file_path: String,
}

impl ServerCommand {
    async fn do_migrate(&self, database_data: &Arc<DatabaseData>) -> Result<(), MigrationError> {
        let migrator_service = MigratorService::from(database_data);
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
        //初始化service provider
        let service_provider = web::Data::new(Provider::new(&database_data));
        //run api server
        println!(
            "api server run at http://{}:{}",
            app_config.server.host, app_config.server.port
        );
        HttpServer::new(move || app_factory(App::new(), &service_provider))
            .bind((app_config.server.host.as_str(), app_config.server.port))?
            .run()
            .await?;
        println!(" - api server stopped");
        Ok(())
    }
}

impl Command for ServerCommand {
    /// 运行 http 服务
    fn execute(&self) {
        //println!("host={} port={}", &self.host, self.port);
        if let Err(err) = rt::System::new().block_on(self.launch()) {
            panic!("{err}")
        }
    }
}

///app工厂函数
pub fn app_factory<T>(app: App<T>, service_provider: &web::Data<Provider>) -> App<T>
where
    T: ServiceFactory<ServiceRequest, Config = (), Error = actix_web::Error, InitError = ()>,
{
    app.configure(routes::configure_routes)
        .configure(|cfg| configure_data(cfg, service_provider))
}

///配置共享数据
fn configure_data(cfg: &mut web::ServiceConfig, service_provider: &web::Data<Provider>) {
    cfg.app_data(service_provider.clone());
}
