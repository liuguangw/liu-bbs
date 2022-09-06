use std::ops::Deref;

///集合名称
pub enum CollectionName {
    ///迁移记录集合
    Migrations,
    ///计数器集合
    Counters,
    ///用户集合
    Users,
    ///用户邮箱绑定记录集合
    UserEmails,
    ///用户会话集合
    Sessions,
}
impl CollectionName {
    ///字符串形式
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Migrations => "migrations",
            Self::Counters => "counters",
            Self::Users => "users",
            Self::UserEmails => "user_emails",
            Self::Sessions => "sessions",
        }
    }
}

impl Deref for CollectionName {
    type Target = str;
    ///获取集合名称
    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}
