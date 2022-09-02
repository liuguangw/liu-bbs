use super::DatabaseConfig;
use mongodb::bson::doc;
use mongodb::options::ClientOptions;
use mongodb::{Client, Database};

///数据库连接对象
pub struct DatabaseData {
    ///数据库对象
    pub database: Database,
    ///client对象
    pub client: Client,
    ///集合前缀
    pub collection_prefix: String,
}

impl DatabaseData {
    ///连接数据库
    pub async fn connect(database_config: &DatabaseConfig) -> Result<Self, mongodb::error::Error> {
        let mut client_options = ClientOptions::parse(&database_config.source).await?;
        let app_name = format!("{}/{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
        client_options.app_name = Some(app_name);
        let client = Client::with_options(client_options)?;
        let database = client.database(&database_config.database_name);
        //ping
        database
            .run_command(
                doc! {
                    "ping":1,
                },
                client.selection_criteria().cloned(),
            )
            .await?;
        Ok(Self {
            database,
            client,
            collection_prefix: database_config.collection_prefix.to_string(),
        })
    }
    ///获取数据库服务器版本
    pub async fn db_version(&self) -> Result<String, mongodb::error::Error> {
        let result = self
            .database
            .run_command(
                doc! {
                   "buildInfo": 1,
                },
                self.client.selection_criteria().cloned(),
            )
            .await?;
        dbg!(&result);
        let version = result.get_str("version").expect("get version failed");
        Ok(version.to_string())
    }
}
