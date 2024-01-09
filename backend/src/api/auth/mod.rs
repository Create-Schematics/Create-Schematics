use oauth2::{AuthorizationCode, TokenResponse};
use oauth2::reqwest::async_http_client;
use poem::web::cookie::CookieJar;
use poem::web::Data;
use poem_openapi::OpenApi;
use poem_openapi::param::{Path, Query};
use rand::Rng;
use reqwest::header;
use uuid::Uuid;

use crate::authentication::oauth::{OauthUser, OauthProvider};
use crate::authentication::session::UserSession;
use crate::error::{ApiError, ResultExt};
use crate::redirect::RedirectResponse;
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

        let user_id = oauth_user.get_or_create_user(provider, &mut transaction).await?;
        let session = UserSession::new_for_user(user_id);
        
        session.save(&ctx.redis_pool).await?;
        cookies.add(session.into_cookie());
        
        transaction.commit().await?;

        Ok(RedirectResponse::to("/"))
    }

    #[oai(path = "/auth/refresh", method = "post")]
    async fn refresh(
        &self,
        Data(ctx): Data<&ApiContext>,
        cookies: &CookieJar
    ) -> ApiResult<()> {
        let session_id = cookies
            .get(UserSession::NAMESPACE)
            .ok_or(ApiError::Unauthorized)?
            .to_string();
        
        let session = UserSession::from_id(session_id, &ctx.redis_pool).await?;
        session.clear(&ctx.redis_pool, &cookies).await?;

        let session = UserSession::new_for_user(session.user_id);

        session.save(&ctx.redis_pool).await?;
        cookies.add(session.into_cookie());

        Ok(())
    }

    #[oai(path = "/auth/logout", method = "post")]
    async fn logout(
        &self,
        Data(ctx): Data<&ApiContext>,
        cookies: &CookieJar
    ) -> ApiResult<()> {
        let session_id = cookies
            .get(UserSession::NAMESPACE)
            .ok_or(ApiError::Unauthorized)?
            .to_string();
        
        let session = UserSession::from_id(session_id, &ctx.redis_pool).await?;
        session.clear(&ctx.redis_pool, &cookies).await?;
        
        Ok(())
    }
}

impl OauthUser {
    pub const MAX_ATTEMPTS: usize = 5;

    async fn get_or_create_user(
        self,
        provider: OauthProvider,
        transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    ) -> ApiResult<Uuid> {
        let provider = provider.to_string();

        let user_meta = sqlx::query!(
            r#"select user_id from users where oauth_provider = $1 and oauth_id = $2"#,
            provider, self.oauth_id
        )
        .fetch_optional(&mut **transaction)
        .await?;

        if let Some(user_meta) = user_meta {
            return Ok(user_meta.user_id)
        } 

        let mut username = self.username.clone();
        let mut attempts = 0;

        
        while Self::is_username_used(&username, transaction).await? {
            // If the username is already used add a random value to the end of the username until it 
            // is unqiue for example Rabbitminers -> Rabbitminers123
            let suffix = rand::thread_rng().gen_range(10..=999);
            username = format!("{username}{suffix}");
            
            attempts += 1;
            
            // Check if attempts are exceeded here to prevent hitting the database again needlessly
            if attempts > Self::MAX_ATTEMPTS {
                return Err(ApiError::InternalServerError)
            }
        }
        
        // If the oauth provider doesnt return a display name use the username before any suffixes are 
        // added to it as display names do not need to be unique unlike usernames.
        let display_name = self.display_name.unwrap_or(self.username);

        let user_id = sqlx::query!(
            r#"
            insert into users (
                username, displayname, 
                email, avatar, oauth_id,
                oauth_provider
            )
            values (
                $1, $2, $3, $4, $5, $6
            )
            returning user_id
            "#,
            username,
            display_name,
            self.email,
            self.avatar_url,
            self.oauth_id,
            provider
        )
        .fetch_one(&mut **transaction)
        .await
        .on_constraint("users_email_key", |_| {
            ApiError::unprocessable_entity([("email", "a user with this email already exists")])
        })?
        .user_id;

        Ok(user_id)
    }

    async fn is_username_used(
        username: &str,
        transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    ) -> ApiResult<bool> {
        let username_used = sqlx::query!(
            r#"select user_id from users where username = $1"#,
            username
        )
        .fetch_optional(&mut **transaction)
        .await?
        .is_some();

        Ok(username_used)
    }
}
