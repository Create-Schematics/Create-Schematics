
use axum::{Router, Extension};
use axum::response::Redirect;
use axum::routing::get;
use axum::extract::{Path, State, Query};
use clap::Args;
use oauth2::{Scope, AuthorizationCode, TokenResponse};
use oauth2::reqwest::async_http_client;
use oauth2::basic::BasicClient;
use reqwest::{header, Response};
use tower_cookies::Cookies;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::authentication::session::Session;
use crate::models::user::Permissions;
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

#[derive(Args, Debug)]
pub struct StartCommandOauthArguments {
    #[cfg(feature="google-oauth")]
    #[command(next_help_heading = "Google")]
    #[command(flatten)]
    pub google: google::GoogleOauthArguments,

    #[cfg(feature="discord-oauth")]
    #[command(next_help_heading = "Discord")]
    #[command(flatten)]
    pub discord: discord::DiscordOauthArguments,

    #[cfg(feature="microsoft-oauth")]
    #[command(next_help_heading = "Microsoft")]
    #[command(flatten)]
    pub microsoft: microsoft::MicrosoftOauthArguments,

    #[cfg(feature="github-oauth")]
    #[command(next_help_heading = "Github")]
    #[command(flatten)]
    pub github: github::GitHubOauthArguments
}

pub (in crate::api) fn configure(
    args: StartCommandOauthArguments
) -> Result<Router<ApiContext>, anyhow::Error> {
    let clients = OauthClients::build(args)?;
    
    let router = Router::new()
        .route("/auth/:provider", get(oauth_authorization))
        .route("/auth/:provider/callback", get(oauth_callback))
        .layer(Extension(clients));
        
    Ok(router)
}

#[derive(Debug, Deserialize, ToSchema)]
pub (in crate::api) struct AuthRequest {
    pub code: String,
    // TODO: Handle CSRF state
    // pub state: String
}


#[derive(Deserialize, Debug, ToSchema)]
#[serde(rename_all="lowercase")]
pub enum OauthProviders {
    #[cfg(feature="github-oauth")]
    GitHub,

    #[cfg(feature="microsoft-oauth")]
    Microsoft,

    #[cfg(feature="google-oauth")]
    Google,
    
    #[cfg(feature="discord-oauth")]
    Discord
}

#[derive(Clone, Debug)]
pub struct OauthClients {
    #[cfg(feature="github-oauth")]
    pub github: BasicClient,
    
    #[cfg(feature="microsoft-oauth")]
    pub microsoft: BasicClient,
    
    #[cfg(feature="google-oauth")]
    pub google: BasicClient,

    #[cfg(feature="discord-oauth")]
    pub discord: BasicClient
}

#[utoipa::path(
    get,
    path = "/auth/{provider}",
    context_path = "/api",
    tag = "authentication",
    params(
        ("provider" = OauthProviders, Path, description = "The oauth provider to authenticate with"),
    ),
    responses(
        (status = 303, description = "Redirecting to oauth provider", headers(("Location" = String))),
        (status = 400, description = "Invalid oauth provider")
    ),
    security(("session_cookie" = []))
)]
async fn oauth_authorization(
    Extension(clients): Extension<OauthClients>,
    Path(provider): Path<OauthProviders>
) -> Redirect {
    let client = clients.get(&provider);
    
    let (auth_url, _csrf_state) = client
        .authorize_url(oauth2::CsrfToken::new_random)
        .add_scopes(provider.scopes())
        .url();

    Redirect::to(&auth_url.to_string())
}

#[utoipa::path(
    get,
    path = "/auth/{provider}/callback",
    context_path = "/api",
    tag = "authentication",
    params(
        ("provider" = OauthProviders, Path, description = "The oauth provider to authenticate with"),
        ("query" = AuthRequest, Query, description = "Current oauth state")
    ),
    responses(
        (status = 303, description = "Redirecting to home page", headers(("Location" = String))),
        (status = 400, description = "Invalid oauth provider"),
        (status = 500, description = "Internal server error while authorizing user")
    ),
    security(("session_cookie" = []))
)]
async fn oauth_callback(
    Extension(clients): Extension<OauthClients>,
    Path(provider): Path<OauthProviders>,
    State(ctx): State<ApiContext>,
    Query(query): Query<AuthRequest>,
    cookies: Cookies
) -> ApiResult<Redirect> {
    let oauth_client = clients.get(&provider);

    let token = oauth_client
        .exchange_code(AuthorizationCode::new(query.code))
        .request_async(async_http_client)
        .await
        .map_err(|e| anyhow::anyhow!(e))?;

    let client = reqwest::Client::new();

    let response = client
        .get(provider.data_url())
        .header(header::USER_AGENT, "Create-Schematics")
        .bearer_auth(token.access_token().secret())
        .send()
        .await
        .map_err(|e| anyhow::anyhow!(e))?;

    let mut transaction = ctx.pool.begin().await?;
    let user = provider.get_or_create_user(response, &mut transaction).await?;

    transaction.commit().await?;
    
    let session = Session::new_for_user(user);
    
    session.save(&ctx.redis_pool).await?;
    cookies.add(session.into_cookie());

    Ok(Redirect::to("/"))
}

impl OauthProviders {
    pub fn scopes(&self) -> Vec<Scope> {
        match self {
            #[cfg(feature="github-oauth")]
            OauthProviders::GitHub => vec![
                Scope::new("read:user".to_string()),
                Scope::new("user:email".to_string())
            ],

            #[cfg(feature="microsoft-oauth")]
            OauthProviders::Microsoft => vec![
                Scope::new("user.read".to_string()),
            ],

            #[cfg(feature="google-oauth")]
            OauthProviders::Google => vec![
                Scope::new("openid".to_string()), 
                Scope::new("email".to_string()), 
                Scope::new("profile".to_string())
            ],

            #[cfg(feature="discord-oauth")]
            OauthProviders::Discord => vec![
                Scope::new("identify".to_string()), 
                Scope::new("email".to_string())
            ],
        }
    }

    pub async fn get_or_create_user(
        &self, 
        user: Response, 
        transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    ) -> ApiResult<Uuid> {
        match self {
            #[cfg(feature="github-oauth")]
            OauthProviders::GitHub => {
                #[derive(Serialize, Deserialize, Debug)]
                pub struct GitHubUser {
                    pub login: String,
                    pub id: u64,
                    pub avatar_url: String,
                    pub name: Option<String>,
                    pub email: Option<String>,
                }

                let github_user: GitHubUser = user
                    .json()
                    .await
                    .map_err(|e| anyhow::anyhow!(e))?;

                let github_id = github_user.id.to_string();

                let user_meta = sqlx::query!(
                    r#"select user_id from users where github_id = $1"#,
                    github_id
                )
                .fetch_optional(&mut **transaction)
                .await?;
                
                if let Some(user_meta) = user_meta {
                    return Ok(user_meta.user_id)
                }

                let username = github_user.name.unwrap_or(github_user.login);
                let permissions = Permissions::default().bits() as i32;

                let user_id = sqlx::query!(
                    r#"
                    insert into users (
                        username, email, avatar,
                        permissions, github_id
                    )
                    values (
                        $1, $2, $3, $4, $5
                    )
                    on conflict (email)
                    do update
                    set github_id = $5
                    returning user_id
                    "#,
                    username,
                    github_user.email,
                    github_user.avatar_url,
                    permissions,
                    github_id
                )
                .fetch_one(&mut **transaction)
                .await?
                .user_id;

                Ok(user_id)
            },

            #[cfg(feature="microsoft-oauth")]
            OauthProviders::Microsoft => {
                #[derive(Deserialize, Debug)]
                #[serde(rename_all = "camelCase")]
                pub struct MicrosoftUser {
                    pub id: String,
                    pub display_name: Option<String>,
                    pub mail: String,
                    pub user_principal_name: String,
                }

                let microsoft_user: MicrosoftUser = user
                    .json()
                    .await
                    .map_err(|e| anyhow::anyhow!(e))?;

                let user_meta = sqlx::query!(
                    r#"select user_id from users where microsoft_id = $1"#,
                    microsoft_user.id
                )
                .fetch_optional(&mut **transaction)
                .await?;

                if let Some(user_meta) = user_meta {
                    return Ok(user_meta.user_id)
                }

                let username = microsoft_user
                    .display_name
                    .unwrap_or(username_from_email(&microsoft_user.user_principal_name));

                let permissions = Permissions::default().bits() as i32;

                let user_id = sqlx::query!(
                    r#"
                    insert into users (
                        username, email,
                        permissions, microsoft_id
                    )
                    values (
                        $1, $2, $3, $4
                    )
                    on conflict (email)
                    do update
                    set microsoft_id = $4
                    returning user_id
                    "#,
                    username,
                    microsoft_user.mail,
                    permissions,
                    microsoft_user.id
                )
                .fetch_one(&mut **transaction)
                .await?
                .user_id;

                Ok(user_id)
            },

            #[cfg(feature="google-oauth")]
            OauthProviders::Google => {
                #[derive(Deserialize, Debug)]
                pub struct GoogleUser {
                    pub id: String,
                    pub email: String,
                    pub name: Option<String>,
                    pub bio: Option<String>,
                    pub picture: Option<String>,
                }

                let google_user: GoogleUser = user
                    .json()
                    .await
                    .map_err(|e| anyhow::anyhow!(e))?;

                let user_meta = sqlx::query!(
                    r#"select user_id from users where google_id = $1"#,
                    google_user.id
                )
                .fetch_optional(&mut **transaction)
                .await?;

                if let Some(user_meta) = user_meta {
                    return Ok(user_meta.user_id)
                }

                let username = google_user
                    .name
                    .unwrap_or(username_from_email(&google_user.email));

                let permissions = Permissions::default().bits() as i32;

                let user_id = sqlx::query!(
                    r#"
                    insert into users (
                        username, email, avatar,
                        permissions, google_id
                    )
                    values (
                        $1, $2, $3, $4, $5
                    )
                    on conflict (email)
                    do update
                    set google_id = $5
                    returning user_id
                    "#,
                    username,
                    google_user.email,
                    google_user.picture,
                    permissions,
                    google_user.id
                )
                .fetch_one(&mut **transaction)
                .await?
                .user_id;

                Ok(user_id)
            },

            #[cfg(feature="discord-oauth")]
            OauthProviders::Discord => {
                #[derive(Serialize, Deserialize, Debug)]
                pub struct DiscordUser {
                    pub id: String,
                    pub username: String,
                    pub avatar: Option<String>,
                    pub global_name: Option<String>,
                    pub email: String,
                }

                let discord_user: DiscordUser = user
                    .json()
                    .await
                    .map_err(|e| anyhow::anyhow!(e))?;
                
                let user_meta = sqlx::query!(
                    r#"select user_id from users where discord_id = $1"#,
                    discord_user.id
                )
                .fetch_optional(&mut **transaction)
                .await?;

                if let Some(user_meta) = user_meta {
                    return Ok(user_meta.user_id)
                }

                let username = discord_user
                    .global_name
                    .unwrap_or(discord_user.username);

                let avatar = discord_user
                    .avatar
                    .map(|x| format!("https://cdn.discordapp.com/avatars/{}/{}.webp", discord_user.id, x));

                let permissions = Permissions::default().bits() as i32;

                let user_id = sqlx::query!(
                    r#"
                    insert into users (
                        username, email, avatar,
                        permissions, discord_id
                    )
                    values (
                        $1, $2, $3, $4, $5
                    )
                    on conflict (email)
                    do update
                    set discord_id = $5
                    returning user_id
                    "#,
                    username,
                    discord_user.email,
                    avatar,
                    permissions,
                    discord_user.id
                )
                .fetch_one(&mut **transaction)
                .await?
                .user_id;

                Ok(user_id)
            },
        }
    }


    pub fn data_url(&self) -> &str {
        match self {
            OauthProviders::GitHub => "https://api.github.com/user",
            OauthProviders::Microsoft => "https://graph.microsoft.com/v1.0/me?$select=id,displayName,mail,userPrincipalName",
            OauthProviders::Google => "https://www.googleapis.com/oauth2/v3/userinfo",
            OauthProviders::Discord => "https://discordapp.com/api/users/@me",
        }
    }
}

pub fn username_from_email(email: &str) -> String {
    email.split('@').next().unwrap_or_default().to_string()
}

impl OauthClients {
    pub fn build(args: StartCommandOauthArguments) -> Result<Self, anyhow::Error> {
        Ok(Self {
            #[cfg(feature="github-oauth")]
            github: github::build_client(args.github)?,

            #[cfg(feature="microsoft-oauth")]
            microsoft: microsoft::build_client(args.microsoft)?,

            #[cfg(feature="google-oauth")]
            google: google::build_client(args.google)?,

            #[cfg(feature="discord-oauth")]
            discord: discord::build_client(args.discord)?
        })
    }
 
    pub fn get(&self, provider: &OauthProviders) -> &BasicClient {
        match provider {
            #[cfg(feature="github-oauth")]
            OauthProviders::GitHub => &self.github,
            
            #[cfg(feature="microsoft-oauth")]
            OauthProviders::Microsoft => &self.microsoft,
            
            #[cfg(feature="google-oauth")]
            OauthProviders::Google => &self.google,

            #[cfg(feature="discord-oauth")]
            OauthProviders::Discord => &self.discord
        }
    }
}