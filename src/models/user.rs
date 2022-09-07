use rand::RngCore;
use serde::{Deserialize, Serialize};
use sha1::{Digest, Sha1};
use std::fmt::Write;
use std::time::SystemTime;

///用户
#[derive(Serialize, Deserialize)]
pub struct User {
    ///用户id
    #[serde(rename = "_id")]
    pub id: i64,
    ///用户名
    pub username: String,
    ///昵称
    pub nickname: String,
    ///hash加密过的密码
    password_hash: String,
    ///加密所需的salt参数
    password_salt: String,
    ///头像
    pub avatar: String,
    ///经验值
    pub exp: i64,
    ///金币值
    pub coin: i64,
    ///已验证邮箱
    pub status_email_verified: bool,
    ///已验证手机
    pub status_mobile_verified: bool,
    ///注册时的IP地址
    pub register_ip: String,
    ///注册时间
    #[serde(with = "crate::common::serde_helpers::bson_system_time")]
    pub created_at: SystemTime,
    ///更新时间
    #[serde(with = "crate::common::serde_helpers::bson_system_time")]
    pub updated_at: SystemTime,
}

impl Default for User {
    fn default() -> Self {
        let time_now = SystemTime::now();
        Self {
            id: 0,
            username: Default::default(),
            nickname: Default::default(),
            password_hash: Default::default(),
            password_salt: Default::default(),
            avatar: Default::default(),
            exp: 0,
            coin: 0,
            status_email_verified: false,
            status_mobile_verified: false,
            register_ip: Default::default(),
            created_at: time_now,
            updated_at: time_now,
        }
    }
}

impl User {
    ///密码hash加密算法
    fn encrypt_password(password: &str, salt: &str) -> String {
        let input_text = format!("{}{}", password, salt);
        let mut hasher = Sha1::new();
        hasher.update(input_text.as_bytes());
        let result = hasher.finalize();
        let mut hash_text = String::with_capacity(result.len() * 2);
        for b in result.as_slice() {
            write!(hash_text, "{:02x}", b).unwrap();
        }
        hash_text
    }
    ///设置密码
    pub fn set_password(&mut self, password: &str) {
        let mut salt_data = [0u8; 4];
        rand::thread_rng().fill_bytes(&mut salt_data);
        let mut salt = String::with_capacity(salt_data.len() * 2);
        for b in salt_data {
            write!(salt, "{:02x}", b).unwrap();
        }
        let password_hash = Self::encrypt_password(password, &salt);
        self.password_hash = password_hash;
        self.password_salt = salt;
    }
    ///检测密码是否正确
    pub fn check_password(&self, password: &str) -> bool {
        let password_hash = Self::encrypt_password(password, &self.password_salt);
        self.password_hash == password_hash
    }
}
