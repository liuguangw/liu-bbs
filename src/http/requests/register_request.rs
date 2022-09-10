use regex::Regex;
use serde::Deserialize;

use crate::common::{ApiError, ApiRequestValidator};

///用户注册请求
#[derive(Deserialize, Default)]
#[serde(default)]
pub struct RegisterRequest {
    ///会话id
    #[serde(rename = "sid")]
    pub session_id: String,
    ///验证码
    pub captcha_code: String,
    ///用户名
    pub username: String,
    ///昵称
    pub nickname: String,
    ///密码
    pub password: String,
}

impl ApiRequestValidator for RegisterRequest {
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
            return Err(ApiError::new_bad_request(
                "用户名只能包含小写字母、数字、下划线, 并且不能以数字开始",
            ));
        }
        let item_length = self.username.len();
        if item_length < 5 {
            return Err(ApiError::new_bad_request("用户名至少需要5个字符"));
        } else if item_length > 16 {
            return Err(ApiError::new_bad_request("用户名不能超过16位"));
        }
        if self.nickname.is_empty() {
            return Err(ApiError::new_bad_request("昵称不能为空"));
        }
        let item_length = self.nickname.chars().count();
        if item_length < 2 {
            return Err(ApiError::new_bad_request("昵称至少需要5个字符"));
        } else if item_length > 16 {
            return Err(ApiError::new_bad_request("昵称不能超过16位"));
        }
        if self.password.is_empty() {
            return Err(ApiError::new_bad_request("密码不能为空"));
        }
        if !self.password.is_ascii() {
            return Err(ApiError::new_bad_request("密码只能使用ascii字符"));
        }
        let item_length = self.password.len();
        if item_length < 6 {
            return Err(ApiError::new_bad_request("密码至少需要6个字符"));
        } else if item_length > 23 {
            return Err(ApiError::new_bad_request("密码不能超过23位"));
        }
        Ok(())
    }
}
