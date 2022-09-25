use crate::common::ApiRequestValidator;
use serde::Deserialize;

///帖子列表筛选参数
#[derive(Deserialize)]
#[serde(default)]
pub struct FilterTopicListRequest {
    ///排序方式
    pub sort_type: u8,
    ///每页最多条数
    pub per_page: i64,
    ///当前页
    pub page: i64,
}

impl Default for FilterTopicListRequest {
    fn default() -> Self {
        Self {
            sort_type: 1,
            per_page: 20,
            page: 1,
        }
    }
}

impl ApiRequestValidator for FilterTopicListRequest {
    fn check_input(&self) -> Result<(), crate::common::ApiError> {
        todo!()
    }
}
