use serde::Deserialize;

///附带会话id的请求
#[derive(Deserialize, Default)]
#[serde(default)]
pub struct SessionRequest {
    ///会话id
    #[serde(rename = "sid")]
    pub id: String,
}
