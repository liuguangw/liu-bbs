use crate::{
    common::{ApiRequest, ResponseResult},
    http::requests::{AuthSessionRequest, SaveTopicRequest},
    models::{Topic, TopicContent},
    services::{TopicService, UserService},
};
use actix_web::{
    post,
    web::{self, Json},
};

///发帖或保存帖子为草稿
#[post("/forums/{forum_id}/topics")]
pub async fn post_topic(
    req: ApiRequest<Json<SaveTopicRequest>>,
    path: web::Path<i64>,
    session_req: AuthSessionRequest,
    user_service: web::Data<UserService>,
    topic_service: web::Data<TopicService>,
) -> ResponseResult<Option<()>> {
    let user_id = session_req.user_id;
    user_service.load_user_info_by_id(user_id).await?;
    let forum_id = path.into_inner();
    let req = req.0.into_inner();
    let mut topic_info = Topic {
        author_user_id: user_id,
        forum_id,
        title: req.title,
        is_publish: req.is_publish,
        ..Default::default()
    };
    if req.is_publish {
        topic_info.publish_at = Some(topic_info.created_at);
    }
    let mut topic_content = TopicContent {
        id: 0,
        content: req.content,
    };
    topic_service
        .process_post_topic(&mut topic_info, &mut topic_content)
        .await?;
    Ok(None.into())
}
