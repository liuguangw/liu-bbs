use serde::Deserialize;

///附带会话id的请求
#[derive(Deserialize)]
pub struct SessionRequest {
    ///会话id
    #[serde(rename = "sid")]
    pub id: String,
}
