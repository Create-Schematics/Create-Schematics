use std::{time::Duration, str::FromStr};

use poem::web::cookie::{SameSite, CookieJar, Cookie};
use uuid::Uuid;

use crate::{helpers::cookies::CookieBuilder, database::redis::RedisPool, response::ApiResult, error::ApiError};

const DEFAULT_SESSION_LENGTH: u64 = 7 * 24 * 60 * 60; // One Week
const TOKEN_LENGTH: usize = 24;

#[derive(Clone, Debug)]
pub (crate) struct UserSession {
    session_id: String,
    pub user_id: Uuid
}

impl UserSession {
    pub const NAMESPACE: &'static str = "session";

    pub fn new_for_user(user_id: Uuid) -> UserSession {
        let session_id = nanoid::nanoid!(TOKEN_LENGTH);

        Self { session_id, user_id }
    }

    pub async fn from_id(
        session_id: String,
        redis_pool: &RedisPool
    ) -> ApiResult<Self> {
        let user = redis_pool
            .get::<String, _>(Self::NAMESPACE, &session_id)
            .await?
            .ok_or(ApiError::Unauthorized)?;

        let user_id = Uuid::from_str(&user).map_err(anyhow::Error::new)?;

        Ok(Self { session_id, user_id })
    }

    pub fn into_cookie(self) -> Cookie {
        CookieBuilder::new(Self::NAMESPACE, self.session_id)
            .path("/")
            .http_only(true)
            .secure(true)
            .same_site(SameSite::Lax)
            .max_age(Duration::from_secs(DEFAULT_SESSION_LENGTH))
            .build()
    }
    
    pub fn take_from_jar(jar: &CookieJar) {
        jar.remove(Self::NAMESPACE);
    }

    pub async fn save(
        &self, 
        redis_pool: &RedisPool
    ) -> ApiResult<()> {
        redis_pool
            .set(Self::NAMESPACE, &self.session_id, &self.user_id.to_string(), Some(DEFAULT_SESSION_LENGTH))
            .await?;

        Ok(())
    }

    pub (crate) async fn clear(
        &self,
        redis_pool: &RedisPool,
        jar: &CookieJar
    ) -> ApiResult<()> {
        redis_pool.delete(Self::NAMESPACE, &self.session_id).await?;
        Self::take_from_jar(&jar);

        Ok(())
    }
}
