use crate::{data::SessionRepository, models::Session};
///会话服务
pub struct SessionService {
    session_repo: SessionRepository,
}

impl SessionService {
    ///构造函数
    pub fn new(session_repo: SessionRepository) -> Self {
        Self { session_repo }
    }
    ///创建新会话
    pub async fn create_new_session(&self, user_id: i64) -> mongodb::error::Result<Session> {
        let mut session = Session::new(user_id);
        self.session_repo.save_session(&mut session).await?;
        Ok(session)
    }

    ///加载会话数据
    pub async fn load_session(&self, session_id: &str) -> mongodb::error::Result<Option<Session>> {
        self.session_repo.find_by_id(session_id).await
    }
}
