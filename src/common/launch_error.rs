use super::AppConfigError;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};
/// 服务启动错误
#[derive(Debug)]
pub enum LaunchError {
    ///配置加载错误
    ConfigError(AppConfigError),
    ///服务启动错误
    RocketError(Box<rocket::Error>),
}
impl From<rocket::Error> for LaunchError {
    fn from(e: rocket::Error) -> Self {
        Self::RocketError(Box::new(e))
    }
}
impl From<AppConfigError> for LaunchError {
    fn from(e: AppConfigError) -> Self {
        Self::ConfigError(e)
    }
}

impl Display for LaunchError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match &self {
            Self::RocketError(e) => <rocket::Error as Display>::fmt(e, f),
            Self::ConfigError(e) => <AppConfigError as Display>::fmt(e, f),
        }
    }
}

impl std::error::Error for LaunchError {}
