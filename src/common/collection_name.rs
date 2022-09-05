use std::ops::Deref;

///集合名称
pub enum CollectionName {
    ///迁移记录集合
    Migrations,
    ///计数器集合
    Counters,
}
impl CollectionName {
    ///字符串形式
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Migrations => "migrations",
            Self::Counters => "counters",
        }
    }
}

impl Deref for CollectionName {
    type Target = str;
    ///获取集合名称
    fn deref(&self) -> &str {
        self.as_str()
    }
}
