use crate::{common::DatabaseResult, data::SessionRepository, models::Session};
use uuid::Uuid;
///会话服务
pub struct SessionService {
    session_repo: SessionRepository,
}

impl SessionService {
    ///构造函数
    pub fn new(session_repo: SessionRepository) -> Self {
        Self { session_repo }
    }

    ///加载会话数据
    pub async fn load_session(&self, session_id: &str) -> DatabaseResult<Option<Session>> {
        self.session_repo
            .find_by_id(session_id)
            .await
            .map_err(|e| e.into())
    }

    ///生成随机id
    fn generate_random_id() -> String {
        let id = Uuid::new_v4().simple();
        id.encode_lower(&mut Uuid::encode_buffer()).to_string()
    }

    ///创建新会话,并保存到数据库
    pub async fn create_new_session(&self, user_id: Option<i64>) -> DatabaseResult<Session> {
        let mut session = Session::new(user_id);
        session.id = Self::generate_random_id();
        self.session_repo.insert_session(&session).await?;
        Ok(session)
    }

    ///更新会话数据
    pub async fn update_session(&self, session: &Session) -> DatabaseResult<()> {
        self.session_repo.update_session(session).await?;
        Ok(())
    }
}
