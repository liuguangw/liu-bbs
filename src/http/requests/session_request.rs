use crate::common::{ApiError, ApiRequestValidator};
use serde::Deserialize;

///附带会话id的请求
#[derive(Deserialize, Default)]
#[serde(default)]
pub struct SessionRequest {
    ///会话id
    #[serde(rename = "sid")]
    pub session_id: String,
}
impl ApiRequestValidator for SessionRequest {
    ///检测用户输入
    fn check_input(&self) -> Result<(), ApiError> {
        if self.session_id.is_empty() {
            return Err(ApiError::new_bad_request("会话id不能为空"));
        }
        Ok(())
    }
}
