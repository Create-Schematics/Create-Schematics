use axum::{Extension, RequestPartsExt};
use axum::extract::{FromRequestParts, FromRef}; 
use axum::http::request::Parts;
use tower_cookies::Cookies;

use crate::api::ApiContext;
use crate::authentication::session::Session;
use crate::error::ApiError;
use crate::models::user::User;

#[axum::async_trait]
impl<S> FromRequestParts<S> for Session
where
    S: Send + Sync,
    ApiContext: FromRef<S>
{
    type Rejection = ApiError;

    async fn from_request_parts(
        parts: &mut Parts, 
        state: &S
    ) -> Result<Self, Self::Rejection> {
        let Extension(cookies) = parts
            .extract::<Extension<Cookies>>()
            .await
            .map_err(|e| anyhow::anyhow!(e))?;
        
        let ctx = ApiContext::from_ref(state);

        Session::from_jar(cookies, ctx.redis_pool).await
    }
}

#[axum::async_trait]
impl<S> FromRequestParts<S> for User
where
    S: Send + Sync,
    ApiContext: FromRef<S>
{
    type Rejection = ApiError;

    async fn from_request_parts(
        parts: &mut Parts, 
        state: &S
    ) -> Result<Self, Self::Rejection> {
        let session = Session::from_request_parts(parts, state).await?;

        let ctx = ApiContext::from_ref(state);

        let user=  sqlx::query_as!(
            User,
            r#"
            select
                user_id, username,
                permissions, email
            from
                users
            where
                user_id = $1
            "#,
            session.user_id
        )
        .fetch_optional(&ctx.pool)
        .await?
        .ok_or(ApiError::Forbidden)?; // This should be impossible

        Ok(user)
    }
}