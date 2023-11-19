use std::str::FromStr;

use time::Duration;
use tower_cookies::{Cookies, cookie::SameSite, Cookie};
use uuid::Uuid;

use crate::database::redis::RedisPool;
use crate::response::ApiResult;
use crate::error::ApiError;

const DEFAULT_SESSION_LENGTH: i64 = 7 * 24 * 60 * 60; // One Week
const SESSION_ID_LENGTH: usize = 16;

pub (crate) struct Session {
    session_id: String,
    pub user_id: Uuid
}

impl Session {
    pub const NAMESPACE: &str = "session";

    pub (crate) fn new_for_user(user_id: Uuid) -> Session {
        let session_id = nanoid::nanoid!(SESSION_ID_LENGTH);

        Self { session_id, user_id }
    }

    pub (crate) async fn from_jar(
        jar: Cookies,
        redis_pool: RedisPool
    ) -> ApiResult<Self> {
        let session_id = jar.get(Self::NAMESPACE)
            .ok_or(ApiError::Unauthorized)?
            .value()
            .to_string();

        let user = redis_pool
            .get::<String, _>(Self::NAMESPACE, &session_id)
            .await?
            .ok_or(ApiError::Unauthorized)?;

        let user_id = Uuid::from_str(&user).map_err(|e| anyhow::anyhow!(e))?;

        Ok(Self { session_id, user_id })
    }

    pub (crate) fn into_cookie<'c>(self) -> Cookie<'c> {
        Cookie::build(Self::NAMESPACE, self.session_id.to_string())
            .same_site(SameSite::Strict)
            .secure(false)
            .http_only(true)
            .max_age(Duration::seconds(DEFAULT_SESSION_LENGTH))
            .finish()
    }

    pub (crate) fn take_from_jar(jar: Cookies) {
        jar.remove(Cookie::new(Self::NAMESPACE, ""));
    }

    pub (crate) async fn save(
        &self, 
        redis_pool: RedisPool
    ) -> ApiResult<()> {
        redis_pool
            .set(Self::NAMESPACE, &self.session_id, &self.user_id.to_string(), DEFAULT_SESSION_LENGTH)
            .await?;

        Ok(())
    }

    pub (crate) async fn clear(
        self,
        redis_pool: RedisPool
    ) -> ApiResult<()> {
        redis_pool.delete(Self::NAMESPACE, &self.session_id).await?;
        
        Ok(())
    }

}