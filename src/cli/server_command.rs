use crate::routes;
use clap::Args;
use rocket::Config;

/// 运行 `HTTP` 服务的命令
#[derive(Args)]
pub struct ServerCommand {
    #[clap(short = 'H', long, default_value_t = String::from("127.0.0.1"), value_parser)]
    host: String,
    #[clap(short = 'P', long, default_value_t = 8000, value_parser)]
    port: u16,
}

impl ServerCommand {
    /// 运行 http 服务
    pub fn execute(&self) {
        //println!("host={} port={}", &self.host, self.port);
        if let Err(err) = rocket::execute(self.launch()) {
            panic!("{}", err)
        }
    }
    /// 启动rocket
    async fn launch(&self) -> Result<(), rocket::Error> {
        let figment = Config::figment()
            .merge(("address", &self.host))
            .merge(("port", self.port));
        let _rocket = {
            let r = rocket::custom(figment);
            let r = routes::load_common_routes(r);
            routes::load_api_routes(r)
        }
        .launch()
        .await?;
        Ok(())
    }
}
