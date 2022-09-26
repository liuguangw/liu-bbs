use crate::{
    common::{ApiError, CounterKey, DatabaseError, DatabaseResult},
    data::{CounterRepository, UserRepository},
    models::User,
};
use std::{
    collections::{hash_map::Entry, HashMap},
    sync::Arc,
};

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

    ///处理用户登录
    pub async fn process_login(&self, username: &str, password: &str) -> Result<User, ApiError> {
        //判断用户名是否已经存在
        let user = match self
            .user_repo
            .find_by_username(username)
            .await
            .map_err(DatabaseError::from)?
        {
            Some(v) => v,
            None => {
                let message = format!("用户名 {} 不存在", username);
                return Err(ApiError::Common(message));
            }
        };
        //判断密码是否正确
        if !user.check_password(password) {
            Err(ApiError::Common("密码错误".to_string()))
        } else {
            Ok(user)
        }
    }

    ///通过用户id加载用户信息
    pub async fn load_option_user_info_by_id(&self, user_id: i64) -> DatabaseResult<Option<User>> {
        self.user_repo
            .find_by_id(user_id)
            .await
            .map_err(|e| e.into())
    }

    ///通过用户id加载用户信息
    pub async fn load_user_info_by_id(&self, user_id: i64) -> Result<User, ApiError> {
        let option_user_info = self.load_option_user_info_by_id(user_id).await?;
        match option_user_info {
            Some(user_info) => Ok(user_info),
            None => {
                let message = format!("用户id: {}不存在", user_id);
                Err(ApiError::Common(message))
            }
        }
    }
}

///使用缓存的用户信息
pub struct UserInfoState<'a> {
    user_info_map: HashMap<i64, Option<User>>,
    user_service: &'a UserService,
}

impl<'a> UserInfoState<'a> {
    ///构造函数
    pub fn new(user_service: &'a UserService) -> Self {
        Self {
            user_info_map: HashMap::new(),
            user_service,
        }
    }
    ///通过用户id加载用户信息
    ///
    /// 在需要多次查询用户信息时使用, 不会重复查询同一个用户id,优先使用map。
    pub async fn load_user_info(&mut self, user_id: i64) -> DatabaseResult<Option<&User>> {
        //如果不存在,则插入
        if let Entry::Vacant(entry) = self.user_info_map.entry(user_id) {
            let db_user_info = self
                .user_service
                .load_option_user_info_by_id(user_id)
                .await?;
            entry.insert(db_user_info);
            //dbg!(user_id);
        };
        let user_info = self.user_info_map.get(&user_id).unwrap();
        Ok(user_info.as_ref())
    }
}
