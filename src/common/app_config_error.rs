use std::fmt;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug)]
enum AppConfigInnerError {
    ///配置文件读取错误
    IoErr(std::io::Error),
    ///配置文件解析错误
    ParseErr(toml::de::Error),
}

///配置加载错误
#[derive(Debug)]
pub struct AppConfigError {
    file_path: String,
    inner_error: AppConfigInnerError,
}

impl From<(&str, std::io::Error)> for AppConfigError {
    fn from((file_path, err): (&str, std::io::Error)) -> Self {
        Self {
            file_path: file_path.to_string(),
            inner_error: AppConfigInnerError::IoErr(err),
        }
    }
}

impl From<(&str, toml::de::Error)> for AppConfigError {
    fn from((file_path, err): (&str, toml::de::Error)) -> Self {
        Self {
            file_path: file_path.to_string(),
            inner_error: AppConfigInnerError::ParseErr(err),
        }
    }
}

impl Display for AppConfigError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match &self.inner_error {
            AppConfigInnerError::IoErr(e) => write!(f, "read {} failed, {}", &self.file_path, e),
            AppConfigInnerError::ParseErr(e) => {
                write!(f, "parse {} failed, {}", &self.file_path, e)
            }
        }
    }
}

impl std::error::Error for AppConfigError {}
