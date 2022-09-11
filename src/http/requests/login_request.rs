use crate::common::{ApiError, ApiRequestValidator};
use regex::Regex;
use serde::Deserialize;

///用户登录请求
#[derive(Deserialize, Default)]
#[serde(default)]
pub struct LoginRequest {
    ///会话id
    #[serde(rename = "sid")]
    pub session_id: String,
    ///验证码
    pub captcha_code: String,
    ///用户名
    pub username: String,
    ///密码
    pub password: String,
}

impl ApiRequestValidator for LoginRequest {
    ///检测用户输入
    fn check_input(&self) -> Result<(), ApiError> {
        if self.session_id.is_empty() {
            return Err(ApiError::new_bad_request("会话id不能为空"));
        }
        if self.captcha_code.is_empty() {
            return Err(ApiError::new_bad_request("验证码不能为空"));
        }
        if self.username.is_empty() {
            return Err(ApiError::new_bad_request("用户名不能为空"));
        }
        let username_pattern = Regex::new("^[a-z_][a-z0-9_]+$").unwrap();
        if !username_pattern.is_match(&self.username) {
            return Err(ApiError::new_bad_request("用户名格式不正确"));
        }
        let item_length = self.username.len();
        if !(5..=16).contains(&item_length) {
            return Err(ApiError::new_bad_request("用户名长度不正确"));
        }
        if self.password.is_empty() {
            return Err(ApiError::new_bad_request("密码不能为空"));
        }
        if !self.password.is_ascii() {
            return Err(ApiError::new_bad_request("密码格式错误"));
        }
        let item_length = self.password.len();
        if !(6..=23).contains(&item_length) {
            return Err(ApiError::new_bad_request("密码长度错误"));
        }
        Ok(())
    }
}
