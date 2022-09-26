use crate::{
    common::{ApiRequest, ResponseResult},
    http::{
        requests::FilterTopicListRequest,
        responses::{AuthorInfoNode, PaginationInfo, TopicListNode, TopicListResponse},
    },
    services::{TopicService, UserInfoState, UserService},
};
use actix_web::{get, web};

///论坛的帖子列表
#[get("/forums/{forum_id}/topics")]
pub async fn index(
    path: web::Path<i64>,
    req: ApiRequest<web::Query<FilterTopicListRequest>>,
    topic_service: web::Data<TopicService>,
    user_service: web::Data<UserService>,
) -> ResponseResult<TopicListResponse> {
    //帖子总数量
    let forum_id = path.into_inner();
    let topic_count = topic_service.get_forum_topic_count(forum_id).await?;
    //计算分页数据
    let req = req.0.into_inner();
    let pagination_info = PaginationInfo::from_total_count(topic_count, req.per_page, req.page);
    //获取帖子列表
    let offset = pagination_info.offset();
    let topic_list = topic_service
        .get_forum_topic_list(
            forum_id,
            req.sort_type,
            offset,
            pagination_info.per_page as i64,
        )
        .await?;
    //加入用户信息
    let mut topic_node_list = vec![];
    let mut user_info_state = UserInfoState::new(&user_service);
    for topic_info in topic_list {
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
        let topic_info_node = TopicListNode {
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
        topic_node_list.push(topic_info_node);
    }
    //response
    let topic_list_response = TopicListResponse {
        topic_list: topic_node_list,
        pagination_info,
    };
    Ok(topic_list_response.into())
}
