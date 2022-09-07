use serde::{Deserialize, Serialize};

///计数器
#[derive(Serialize, Deserialize)]
pub struct Counter {
    ///唯一id标识
    #[serde(rename = "_id")]
    pub id: String,
    ///值
    pub value: i64,
}
