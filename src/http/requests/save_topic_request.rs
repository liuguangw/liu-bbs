use crate::common::{ApiError, ApiRequestValidator};
use serde::Deserialize;

///保存主题帖请求
#[derive(Deserialize, Default)]
#[serde(default)]
pub struct SaveTopicRequest {
    ///标题
    pub title: String,
    ///内容
    pub content: String,
    ///是否发布
    pub is_publish: bool,
}
impl ApiRequestValidator for SaveTopicRequest {
    ///检测用户输入
    fn check_input(&self) -> Result<(), ApiError> {
        if self.title.is_empty() {
            return Err(ApiError::new_bad_request("标题不能为空"));
        }
        if self.title.chars().count() > 40 {
            return Err(ApiError::new_bad_request("标题不能超过40个字符"));
        }
        if self.content.is_empty() {
            return Err(ApiError::new_bad_request("内容不能为空"));
        }
        if self.content.chars().count() > 50000 {
            return Err(ApiError::new_bad_request("内容不能超过50000个字符"));
        }
        Ok(())
    }
}
