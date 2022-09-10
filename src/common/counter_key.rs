///计数器key定义
pub enum CounterKey {
    ///最后一个注册用户id
    LastUserId,
}

impl CounterKey {
    ///字符串形式
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::LastUserId => "users.last_id",
        }
    }
}
