use super::app_command::AppCommand;
use crate::common::{AppConfig, DatabaseData, LaunchError, RocketExt};
use clap::Args;
use rocket::figment::Figment;
use rocket::Config;

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
    ///加载rocket配置
    fn load_figment(&self, app_config: &AppConfig) -> Figment {
        //merge 配置项
        let mut config_figment = Config::figment()
            .merge(("address", &app_config.server.host))
            .merge(("port", &app_config.server.port));
        //cli传参可以覆盖配置文件中的定义
        if let Some(ref address) = self.host {
            config_figment = config_figment.merge(("address", address))
        }
        if let Some(port) = self.port {
            config_figment = config_figment.merge(("port", port))
        }
        config_figment
    }
    /// 启动rocket
    async fn launch(&self) -> Result<(), LaunchError> {
        let app_config = AppConfig::load(&self.config_file_path).await?;
        let figment = self.load_figment(&app_config);
        let database_data = DatabaseData::connect(&app_config.database).await?;
        let _rocket = rocket::custom(figment)
            .manage(database_data)
            .add_routes()
            .add_services()
            .launch()
            .await?;
        Ok(())
    }
}

impl AppCommand for ServerCommand {
    /// 运行 http 服务
    fn execute(&self) {
        //println!("host={} port={}", &self.host, self.port);
        if let Err(err) = rocket::execute(self.launch()) {
            panic!("{}", err)
        }
    }
}
