use crate::{
    common::{ApiError, ResponseResult},
    http::responses::{AuthorInfoNode, TopicInfoResponse, TopicListNode},
    services::{Provider, UserInfoState},
};
use actix_web::{get, web};

///帖子详情
#[get("/forum-topics/{topic_id}")]
pub async fn show(
    path: web::Path<i64>,
    service_provider: web::Data<Provider>,
) -> ResponseResult<TopicInfoResponse> {
    //获取帖子信息
    let topic_id = path.into_inner();
    let topic_info = match service_provider
        .topic_service
        .load_topic_info_by_id(topic_id)
        .await?
    {
        Some(s) => s,
        None => {
            let message = format!("帖子id: {}不存在", topic_id);
            return Err(ApiError::Common(message));
        }
    };
    //帖子状态检测
    if !topic_info.is_publish {
        let message = format!("帖子id: {}不存在", topic_id);
        return Err(ApiError::Common(message));
    }
    if topic_info.is_delete {
        let message = format!("帖子id: {}已经被删除了", topic_id);
        return Err(ApiError::Common(message));
    }
    if topic_info.is_block {
        let message = format!("帖子id: {}已经被屏蔽了", topic_id);
        return Err(ApiError::Common(message));
    }
    //content info
    let topic_content_info = match service_provider
        .topic_service
        .load_topic_content_by_id(topic_id)
        .await?
    {
        Some(s) => s,
        None => {
            let message = format!("帖子id: {}内容不存在", topic_id);
            return Err(ApiError::Common(message));
        }
    };
    //meta
    let mut user_info_state = UserInfoState::new(&service_provider.user_service);
    //作者信息
    let author_user_id = topic_info.author_user_id;
    let author_info = user_info_state.load_user_info(author_user_id).await?;
    let author_info = author_info.map(AuthorInfoNode::from);
    //最后一个回帖的用户信息
    let last_reply_user_id = topic_info.last_reply_user_id;
    let last_reply_user_info = match last_reply_user_id {
        Some(tmp_user_id) => user_info_state.load_user_info(tmp_user_id).await?,
        None => None,
    };
    let last_reply_user_info = last_reply_user_info.map(AuthorInfoNode::from);
    let topic_meta_info = TopicListNode {
        id: topic_info.id,
        author_user_id,
        author_info,
        last_reply_user_id,
        last_reply_user_info,
        title: topic_info.title,
        is_publish: topic_info.is_publish,
        is_lock: topic_info.is_lock,
        is_block: topic_info.is_block,
        is_delete: topic_info.is_delete,
        view_count: topic_info.view_count,
        reply_count: topic_info.reply_count,
        publish_at: topic_info.publish_at,
        last_reply_at: topic_info.last_reply_at,
        created_at: topic_info.created_at,
        updated_at: topic_info.updated_at,
    };
    let response_data = TopicInfoResponse {
        meta_info: topic_meta_info,
        content: topic_content_info.content,
    };
    Ok(response_data.into())
}
