use thiserror::Error;
///配置加载错误
#[derive(Debug, Error)]
pub enum AppConfigError {
    ///配置文件读取错误
    #[error("read {0} failed, {1}")]
    IoErr(String, std::io::Error),
    ///配置文件解析错误
    #[error("parse {0} failed, {1}")]
    ParseErr(String, toml::de::Error),
}
