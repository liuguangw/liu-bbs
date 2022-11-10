use std::sync::Arc;

use crate::{
    common::{ApiError, DatabaseResult},
    data::{Provider as DataProvider, SessionRepository},
    models::Session,
};
use uuid::Uuid;
///会话服务
pub struct SessionService {
    data_provider: Arc<DataProvider>,
}

impl From<&Arc<DataProvider>> for SessionService {
    fn from(item: &Arc<DataProvider>) -> Self {
        Self {
            data_provider: item.clone(),
        }
    }
}

impl SessionService {
    #[inline]
    fn session_repo(&self) -> &SessionRepository {
        &self.data_provider.session_repo
    }
    ///加载会话数据
    pub async fn load_session(&self, session_id: &str) -> DatabaseResult<Option<Session>> {
        self.session_repo()
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
        let mut session = match user_id {
            Some(uid) => Session::new(uid),
            None => Default::default(),
        };
        session.id = Self::generate_random_id();
        self.session_repo().insert_session(&session).await?;
        Ok(session)
    }

    ///更新会话数据
    pub async fn update_session(&self, session: &Session) -> DatabaseResult<()> {
        self.session_repo().update_session(session).await?;
        Ok(())
    }

    ///写入验证码到数据库
    pub async fn set_captcha_code(
        &self,
        session: &mut Session,
        captcha_code: String,
    ) -> DatabaseResult<()> {
        session
            .data
            .insert("captcha_code".to_string(), captcha_code);
        self.update_session(session).await
    }
    ///检测验证码是否正确
    pub async fn verify_captcha_code(
        &self,
        session: &mut Session,
        input_code: &str,
    ) -> Result<(), ApiError> {
        //验证码只能校验一次
        let captcha_code = match session.data.remove("captcha_code") {
            Some(v) => v,
            None => return Err(ApiError::new_bad_request("无效的验证码")),
        };
        //update session
        self.update_session(session).await?;
        if captcha_code.eq_ignore_ascii_case(input_code) {
            Ok(())
        } else {
            Err(ApiError::new_bad_request("验证码错误"))
        }
    }
}
