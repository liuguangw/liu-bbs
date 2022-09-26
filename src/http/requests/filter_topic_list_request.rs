use crate::common::{ApiError, ApiRequestValidator};
use serde::Deserialize;

///帖子列表筛选参数
#[derive(Deserialize)]
#[serde(default)]
pub struct FilterTopicListRequest {
    ///排序方式
    pub sort_type: u8,
    ///每页最多条数
    pub per_page: u64,
    ///当前页
    pub page: u64,
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
    fn check_input(&self) -> Result<(), ApiError> {
        if self.per_page == 0 {
            return Err(ApiError::new_bad_request("每页条数必须大于0"));
        } else if self.page > 30 {
            return Err(ApiError::new_bad_request("每页条数必须小于30"));
        }
        if self.page == 0 {
            return Err(ApiError::new_bad_request("条数必须大于0"));
        }
        Ok(())
    }
}
