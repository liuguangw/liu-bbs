use serde::Deserialize;
/// 数据库配置
#[derive(Deserialize)]
#[serde(default)]
pub struct DatabaseConfig {
    ///连接URL
    pub source: String,
    ///数据库名称
    pub database_name: String,
    ///集合前缀
    pub collection_prefix: String,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            source: String::from("mongodb://localhost:27017"),
            database_name: String::from("bbs"),
            collection_prefix: String::from("pre_"),
        }
    }
}
