
use std::fmt::Debug;

use oauth2::{AuthorizationCode, TokenResponse};
use oauth2::reqwest::async_http_client;
use poem::web::cookie::CookieJar;
use poem::web::Data;
use poem_openapi::OpenApi;
use poem_openapi::param::{Path, Query};
use poem_openapi_derive::ApiResponse;
use reqwest::header;
use uuid::Uuid;

use crate::authentication::oauth::{OauthUser, OauthProvider};
use crate::authentication::session::UserSession;
use crate::error::{ResultExt, ApiError};
use crate::response::ApiResult;

use super::ApiContext;

#[cfg(feature="discord-oauth")]
pub mod discord;
#[cfg(feature="google-oauth")]
pub mod google;
#[cfg(feature="microsoft-oauth")]
pub mod microsoft;
#[cfg(feature="github-oauth")]
pub mod github;

pub fn configure() -> impl OpenApi {
    AuthApi
}

pub struct AuthApi;

#[derive(ApiResponse)]
pub enum RedirectResponse {
    #[oai(status = 302)]
    Found(#[oai(header = "location")] String)
}

impl RedirectResponse {
    pub fn to<T>(location: T) -> Self 
    where
        T: Into<String>
    {
        Self::Found(location.into())
    }
}

#[OpenApi]
impl AuthApi {
    #[oai(path = "/auth/:provider", method = "get")]
    async fn oauth_authorization(
        &self,
        Path(provider): Path<OauthProvider>
    ) -> ApiResult<RedirectResponse> {
        let oauth_client = provider.build_client()?;
        
        let (auth_url, _csrf_state) = oauth_client
            .inner
            .authorize_url(oauth2::CsrfToken::new_random)
            .add_scopes(oauth_client.scopes)
            .url();

        Ok(RedirectResponse::to(auth_url))
    }

    #[oai(path = "/auth/:provider/callback", method = "get")]
    async fn oauth_callback(
        &self,
        Data(ctx): Data<&ApiContext>,
        Path(provider): Path<OauthProvider>,
        Query(code): Query<String>,
        cookies: &CookieJar
    ) -> ApiResult<RedirectResponse> {
        let oauth_client = provider.build_client()?;

        let token = oauth_client
            .inner
            .exchange_code(AuthorizationCode::new(code))
            .request_async(async_http_client)
            .await
            .map_err(|e| anyhow::anyhow!(e))?;

        let client = reqwest::Client::new();

        let response = client
            .get(&oauth_client.data_uri)
            .header(header::USER_AGENT, "Create-Schematics")
            .bearer_auth(token.access_token().secret())
            .send()
            .await
            .map_err(|e| anyhow::anyhow!(e))?
            .bytes()
            .await
            .map_err(|e| anyhow::anyhow!(e))?;

        let mut transaction = ctx.pool.begin().await?;
        let oauth_user = oauth_client.extract_user(&response)?;

        let user_id = provider.get_or_create_user(oauth_user, &mut transaction).await?;
        transaction.commit().await?;
        
        let session = UserSession::new_for_user(user_id);

        session.save(&ctx.redis_pool).await?;
        cookies.add(session.into_cookie());

        Ok(RedirectResponse::to("/"))
    }
}

impl OauthProvider {
    async fn get_or_create_user(
        &self,
        user: OauthUser,
        transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    ) -> ApiResult<Uuid> {
        let provider = self.to_string();

        let user_meta = sqlx::query!(
            r#"select user_id from users where oauth_provider = $1 and oauth_id = $2"#,
            provider, user.oauth_id
        )
        .fetch_optional(&mut **transaction)
        .await?;

        if let Some(user_meta) = user_meta {
            return Ok(user_meta.user_id)
        } 

        let username = user.display_name.unwrap_or(user.username);  

        let user_id = sqlx::query!(
            r#"
            insert into users (
                username, email, avatar,
                oauth_provider, oauth_id
            )
            values (
                $1, $2, $3, $4, $5
            )
            returning user_id
            "#,
            username,
            user.email,
            user.avatar_url,
            provider,
            user.oauth_id,
        )
        .fetch_one(&mut **transaction)
        .await
        .on_constraint("users_email_key", |_| {
            ApiError::unprocessable_entity([("email", "a user with this email already exists")])
        })?
        .user_id;

        Ok(user_id)
    }
}
