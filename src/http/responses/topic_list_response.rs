use super::{CommonNodeResponse, ForumInfoResponse};
use crate::models::User;
use serde::Serialize;
use std::time::SystemTime;

///作者信息节点
#[derive(Serialize, Clone)]
pub struct AuthorInfoNode {
    ///用户id
    pub id: i64,
    ///昵称
    pub nickname: String,
    ///头像
    pub avatar: String,
}

///帖子列表节点
#[derive(Serialize)]
pub struct TopicListNode {
    ///帖子id
    pub id: i64,
    ///作者用户id
    pub author_user_id: i64,
    ///作者信息
    pub author_info: Option<AuthorInfoNode>,
    ///最后回复的用户id
    pub last_reply_user_id: Option<i64>,
    ///最后回复的用户信息
    pub last_reply_user_info: Option<AuthorInfoNode>,
    ///标题
    pub title: String,
    ///是否已经发布
    pub is_publish: bool,
    ///是否已锁定
    pub is_lock: bool,
    ///是否已屏蔽
    pub is_block: bool,
    ///是否已删除
    pub is_delete: bool,
    ///阅读数
    pub view_count: i64,
    ///回复数
    pub reply_count: i64,
    ///发布时间
    #[serde(serialize_with = "crate::common::serde_helpers::json_system_time::serialize_option")]
    pub publish_at: Option<SystemTime>,
    ///最后回复时间
    #[serde(serialize_with = "crate::common::serde_helpers::json_system_time::serialize_option")]
    pub last_reply_at: Option<SystemTime>,
    ///创建时间
    #[serde(serialize_with = "crate::common::serde_helpers::json_system_time::serialize")]
    pub created_at: SystemTime,
    ///更新时间
    #[serde(serialize_with = "crate::common::serde_helpers::json_system_time::serialize")]
    pub updated_at: SystemTime,
}

///分页信息
#[derive(Serialize)]
pub struct PaginationInfo {
    ///当前是第几页, n >= 1
    pub current_page: u64,
    ///总共有几页
    pub total_page: u64,
    ///每页最多数据条数
    pub per_page: u64,
    ///当前页数据条数
    pub item_count: u64,
}

///帖子列表响应
#[derive(Serialize)]
pub struct TopicListResponse {
    ///论坛信息
    pub forum_info: ForumInfoResponse,
    ///上级论坛列表
    pub parent_forum_list: Vec<CommonNodeResponse>,
    ///论坛分区信息
    pub forum_group_info: CommonNodeResponse,
    ///分页信息
    pub pagination_info: PaginationInfo,
    ///帖子列表
    pub topic_list: Vec<TopicListNode>,
}

impl PaginationInfo {
    ///从总条数构造
    ///
    /// 需要满足条件 per_page>=1 current_page>=1, 此函数内部不验证这两个条件
    pub fn from_total_count(item_total_count: u64, per_page: u64, current_page: u64) -> Self {
        let mut total_page = item_total_count / per_page;
        if item_total_count % per_page != 0 {
            total_page += 1;
        }
        //current_page不能越界
        let current_page = if total_page == 0 {
            1
        } else if current_page > total_page {
            total_page
        } else {
            current_page
        };
        //计算当前页数据条数
        let item_count = if item_total_count == 0 {
            0
        } else if current_page < total_page {
            per_page
        } else {
            let s = item_total_count % per_page;
            if s == 0 {
                per_page
            } else {
                s
            }
        };
        Self {
            current_page,
            total_page,
            per_page,
            item_count,
        }
    }
    ///计算偏移参数
    pub fn offset(&self) -> u64 {
        (self.current_page - 1) * self.per_page
    }
}

impl From<&User> for AuthorInfoNode {
    fn from(user: &User) -> Self {
        Self {
            id: user.id,
            nickname: user.nickname.to_string(),
            avatar: user.avatar.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::PaginationInfo;

    #[test]
    fn common_pagination() {
        let pagination = PaginationInfo::from_total_count(78, 10, 4);
        assert_eq!(8, pagination.total_page);
        assert_eq!(30, pagination.offset());
        assert_eq!(10, pagination.item_count);
    }

    ///测试最后一页
    #[test]
    fn last_pagination() {
        let pagination = PaginationInfo::from_total_count(78, 10, 8);
        assert_eq!(8, pagination.total_page);
        assert_eq!(70, pagination.offset());
        assert_eq!(8, pagination.item_count);
        let pagination = PaginationInfo::from_total_count(90, 10, 9);
        assert_eq!(9, pagination.total_page);
        assert_eq!(80, pagination.offset());
        assert_eq!(10, pagination.item_count);
    }

    ///测试无数据的情况
    #[test]
    fn zero_pagination() {
        let pagination = PaginationInfo::from_total_count(0, 10, 1);
        assert_eq!(0, pagination.total_page);
        assert_eq!(0, pagination.offset());
        assert_eq!(0, pagination.item_count);
        assert_eq!(1, pagination.current_page);
    }

    ///测试current_page>total_page的参数
    #[test]
    fn out_range_pagination() {
        let pagination = PaginationInfo::from_total_count(78, 10, 15);
        assert_eq!(8, pagination.total_page);
        assert_eq!(70, pagination.offset());
        assert_eq!(8, pagination.item_count);
        assert_eq!(8, pagination.current_page);
        //无数据
        let pagination = PaginationInfo::from_total_count(0, 10, 15);
        assert_eq!(0, pagination.total_page);
        assert_eq!(0, pagination.offset());
        assert_eq!(0, pagination.item_count);
        assert_eq!(1, pagination.current_page);
    }
}
