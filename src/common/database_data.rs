use super::DatabaseConfig;
use mongodb::bson::{doc, Document};
use mongodb::options::ClientOptions;
use mongodb::{Client, Collection, Database};

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
        Self::run_command(
            &database,
            doc! {
                "ping":1,
            },
        )
        .await?;
        Ok(Self {
            database,
            client,
            collection_prefix: database_config.collection_prefix.to_string(),
        })
    }

    ///执行数据库命令
    async fn run_command(
        database: &Database,
        command: Document,
    ) -> Result<Document, mongodb::error::Error> {
        database
            .run_command(command, database.selection_criteria().cloned())
            .await
    }

    ///获取数据库服务器版本
    pub async fn db_version(&self) -> Result<String, mongodb::error::Error> {
        let result = Self::run_command(
            &self.database,
            doc! {
               "buildInfo": 1,
            },
        )
        .await?;
        //dbg!(&result);
        let version = result.get_str("version").expect("get version failed");
        Ok(version.to_string())
    }

    ///获取集合的完整名称
    pub fn collection_full_name(&self, short_name: &str) -> String {
        format!("{}{}", &self.collection_prefix, short_name)
    }

    ///获取集合对象
    pub fn collection<T>(&self, short_name: &str) -> Collection<T> {
        let full_name = self.collection_full_name(short_name);
        self.database.collection(&full_name)
    }
}
