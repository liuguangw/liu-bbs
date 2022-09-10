use crate::{
    common::{ApiError, CounterKey, DatabaseError},
    data::{CounterRepository, UserRepository},
    models::User,
};
use std::sync::Arc;

///用户相关服务
pub struct UserService {
    user_repo: UserRepository,
    counter_repo: Arc<CounterRepository>,
}

impl UserService {
    ///构造函数
    pub fn new(user_repo: UserRepository, counter_repo: &Arc<CounterRepository>) -> Self {
        Self {
            user_repo,
            counter_repo: counter_repo.clone(),
        }
    }
    ///处理用户注册
    pub async fn process_register(&self, user: &mut User) -> Result<(), ApiError> {
        //判断用户名是否已经存在
        let user_tmp = self
            .user_repo
            .find_by_username(&user.username)
            .await
            .map_err(DatabaseError::from)?;
        if user_tmp.is_some() {
            let message = format!("用户名 {} 已存在", user.username);
            return Err(ApiError::Common(message));
        }
        //判断昵称是否已经存在
        let user_tmp = self
            .user_repo
            .find_by_nickname(&user.nickname)
            .await
            .map_err(DatabaseError::from)?;
        if user_tmp.is_some() {
            let message = format!("昵称 {} 已存在", user.nickname);
            return Err(ApiError::Common(message));
        }
        //计算用户id
        let user_id = self
            .counter_repo
            .increment(CounterKey::LastUserId)
            .await
            .map_err(DatabaseError::from)?;
        user.id = user_id;
        //insert
        self.user_repo
            .insert_user(user)
            .await
            .map_err(DatabaseError::from)?;
        Ok(())
    }
}
