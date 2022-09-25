use actix_web::{get, web::Query};

use crate::{
    common::{ApiRequest, ResponseResult},
    http::{requests::FilterTopicListRequest, responses::TopicListResponse},
};

///论坛的帖子列表
#[get("/forums/{forum_id}/topics")]
pub async fn index(
    _req: ApiRequest<Query<FilterTopicListRequest>>,
) -> ResponseResult<TopicListResponse> {
    todo!()
}
