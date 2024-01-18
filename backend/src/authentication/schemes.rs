use poem::Request;
use poem_openapi::auth::ApiKey;
use poem_openapi::payload::Json;
use poem_openapi_derive::SecurityScheme;
use uuid::Uuid;

use crate::api::ApiContext;
use crate::database::redis::RedisPool;
use crate::models::user::User;
use crate::response::ApiResult;
use crate::error::{ApiError, Punishment};

use super::session::UserSession;

pub const TIMEOUT_NAMESPACE: &'static str = "timeout";

#[derive(SecurityScheme)]
#[oai(ty = "api_key", key_name = "session", key_in = "cookie", checker = "session_check")]
pub struct Session(pub Uuid);

async fn session_check(req: &Request, session_id: ApiKey) -> poem::Result<Uuid> {
    let ctx = req.data::<ApiContext>().ok_or(ApiError::InternalServerError)?;
    let session = UserSession::from_id(session_id.key, &ctx.redis_pool).await?;

    check_timeout(&ctx.redis_pool, &session).await?;
    
    Ok(session.user_id)
}

#[derive(SecurityScheme)]
#[oai(ty = "api_key", key_name = "session", key_in = "cookie", checker = "optional_session_check")]
pub struct OptionalSession(pub Option<Uuid>);

async fn optional_session_check(req: &Request, session_id: ApiKey) -> poem::Result<Option<Uuid>> {
    Ok(session_check(req, session_id).await.ok())
}

async fn check_timeout(
    redis_pool: &RedisPool,
    session: &UserSession
) -> ApiResult<()> {
    match redis_pool
        .get_json::<Punishment, _>(TIMEOUT_NAMESPACE, &session.user_id)
        .await? 
    {
        Some(punishment) => Err(ApiError::Banned(Json(punishment))),
        None => Ok(()),
    }
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
                   displayname, about,
                   role, avatar
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
