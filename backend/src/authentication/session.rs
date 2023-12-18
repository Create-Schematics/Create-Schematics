use std::str::FromStr;
use std::time::Duration;

use poem::Request;
use poem::web::cookie::{CookieJar, Cookie, SameSite};
use poem_openapi::auth::ApiKey;
use poem_openapi_derive::SecurityScheme;
use uuid::Uuid;

use crate::api::ApiContext;
use crate::database::redis::RedisPool;
use crate::helpers::cookies::CookieBuilder;
use crate::models::user::User;
use crate::response::ApiResult;
use crate::error::ApiError;

const DEFAULT_SESSION_LENGTH: u64 = 7 * 24 * 60 * 60; // One Week
const SESSION_ID_LENGTH: usize = 24;

#[derive(Clone, Debug)]
pub (crate) struct UserSession {
    session_id: String,
    pub user_id: Uuid
}

#[derive(SecurityScheme)]
#[oai(ty = "api_key", key_name = "session", key_in = "cookie", checker = "session_check")]
pub struct Session(pub Uuid);

async fn session_check(req: &Request, session_id: ApiKey) -> Option<Uuid> {
    let ctx = req.data::<ApiContext>()?;

    let session = UserSession::from_id(session_id.key, &ctx.redis_pool)
        .await
        .ok()?;

    Some(session.user_id)
}

#[derive(SecurityScheme)]
#[oai(ty = "api_key", key_name = "session", key_in = "cookie", checker = "optional_session_check")]
pub struct OptionalSession(pub Option<Uuid>);

async fn optional_session_check(req: &Request, session_id: ApiKey) -> Option<Option<Uuid>> {
    Some(session_check(req, session_id).await)
}

impl Session {
    pub async fn user<'a, E>(
        &self, 
        executor: E
    ) -> ApiResult<User> 
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        sqlx::query_as!(
            User,
            r#"
            select user_id, username,
                   about, role, avatar
            from users
            where user_id = $1
            "#,
            self.0,
        )
        .fetch_optional(executor)
        .await?
        .ok_or(ApiError::Forbidden)
    }

    pub async fn is_moderator<'a, E>(
        &self, 
        executor: E
    ) -> ApiResult<bool> 
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        self.user(executor).await.map(|u| u.is_moderator())
    }

    pub async fn is_administrator<'a, E>(
        &self, 
        executor: E
    ) -> ApiResult<bool> 
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        self.user(executor).await.map(|u| u.is_administrator())
    }

    pub fn user_id(&self) -> Uuid {
        self.0
    }

}

impl UserSession {
    const NAMESPACE: &str = "session";

    pub (crate) fn new_for_user(user_id: Uuid) -> UserSession {
        let session_id = nanoid::nanoid!(SESSION_ID_LENGTH);

        Self { session_id, user_id }
    }

    pub (crate) async fn from_id(
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

    pub (crate) fn into_cookie(self) -> Cookie {
        CookieBuilder::new(Self::NAMESPACE, self.session_id)
            .path("/")
            .http_only(true)
            .secure(true)
            .same_site(SameSite::Lax)
            .max_age(Duration::from_secs(DEFAULT_SESSION_LENGTH))
            .build()
    }
    
    pub (crate) fn _take_from_jar(jar: &CookieJar) {
        jar.remove(Self::NAMESPACE);
    }

    pub (crate) async fn save(
        &self, 
        redis_pool: &RedisPool
    ) -> ApiResult<()> {
        redis_pool
            .set(Self::NAMESPACE, &self.session_id, &self.user_id.to_string(), DEFAULT_SESSION_LENGTH)
            .await?;

        Ok(())
    }

    pub (crate) async fn _clear(
        self,
        redis_pool: &RedisPool
    ) -> ApiResult<()> {
        redis_pool.delete(Self::NAMESPACE, &self.session_id).await?;
        
        Ok(())
    }
}
