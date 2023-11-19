use axum::{Extension, RequestPartsExt};
use axum::extract::{FromRequestParts, FromRef}; 
use axum::http::request::Parts;
use tower_cookies::Cookies;

use crate::authentication::session::Session;
use crate::cli::server::ApiContext; 
use crate::error::ApiError;

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