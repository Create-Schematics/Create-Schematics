use axum::extract::{Query, State};
use axum::response::Redirect;
use axum::routing::get;
use axum::{Router, Extension};
use clap::Args;
use oauth2::reqwest::async_http_client;
use oauth2::{ClientSecret, ClientId, TokenUrl, AuthUrl, RedirectUrl, AuthorizationCode, TokenResponse, CsrfToken, Scope};
use oauth2::basic::BasicClient;
use tower_cookies::Cookies;
use uuid::Uuid;

use crate::api::ApiContext;
use crate::authentication::session::Session;
use crate::response::ApiResult;
use crate::models::user::Permissions;

use super::google::AuthRequest;

#[derive(Args, Debug)]
pub struct DiscordOauthArguments {  
    #[arg(help = "Your discord oauth client id")]
    #[arg(env = "DISCORD_CLIENT_ID", long = "discord-client-id")]
    pub discord_client_id: String,

    #[arg(help = "Your discord oauth client secret")]
    #[arg(env = "DISCORD_CLIENT_SECRET", long = "discord-client-secret")]
    pub discord_client_secret: String,
}

pub (in crate::api::auth) fn configure(
    args: DiscordOauthArguments,
) -> Result<Router<ApiContext>, anyhow::Error> {
    let oauth_client = build_client(args)?;

    let router = Router::new()
        .route("/auth/discord", get(discord_authorization))
        .route("/auth/discord_callback", get(discord_callback))
        .layer(Extension(oauth_client));

    Ok(router)
}

fn build_client(
    DiscordOauthArguments {
        discord_client_id,
        discord_client_secret,
        ..
    }: DiscordOauthArguments
) -> Result<BasicClient, anyhow::Error> {
    let self_address = dotenv::var("SELF_ADDRESS")?;

    let discord_client_secret = ClientSecret::new(discord_client_secret);
    let discord_client_id = ClientId::new(discord_client_id);

    let auth_url = AuthUrl::new("https://discord.com/api/oauth2/authorize?response_type=code".to_string())?;
    let token_url = TokenUrl::new("https://discord.com/api/oauth2/token".to_string())?;

    let redirect_url = RedirectUrl::new(format!("{self_address}/api/auth/discord_callback"))?;

    let client = BasicClient::new(
        discord_client_id,
        Some(discord_client_secret),
        auth_url,
        Some(token_url)
    )
    .set_redirect_uri(redirect_url);

    Ok(client)
}  

#[derive(Debug, Deserialize)]
pub struct DiscordUser {
    id: String,
    avatar: Option<String>,
    email: String,
    username: String
}

async fn discord_authorization(
    Extension(client): Extension<BasicClient>
) -> Redirect {
    let (auth_url, _csrf_token) = client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("identify".to_string()))
        .add_scope(Scope::new("email".to_string()))
        .url();

    Redirect::to(&auth_url.to_string())
}

async fn discord_callback(
    Query(query): Query<AuthRequest>,
    State(ctx): State<ApiContext>,
    cookies: Cookies,
    Extension(oauth_client): Extension<BasicClient>
) -> ApiResult<Redirect> {
    let token = oauth_client
        .exchange_code(AuthorizationCode::new(query.code.clone()))
        .request_async(async_http_client)
        .await
        .map_err(|e| anyhow::anyhow!(e))?;

    let client = reqwest::Client::new();

    let user_data = client
        .get("https://discordapp.com/api/users/@me")
        .header(reqwest::header::USER_AGENT, "Create-Schematics")
        .bearer_auth(token.access_token().secret())
        .send()
        .await
        .map_err(|e| anyhow::anyhow!(e))?
        .json::<DiscordUser>()
        .await
        .map_err(|e| anyhow::anyhow!(e))?;

    let user_id = get_or_create_user(user_data, &ctx).await?;

    let session = Session::new_for_user(user_id);

    session.save(&ctx.redis_pool).await?;
    cookies.add(session.into_cookie());

    Ok(Redirect::to("/"))
}

async fn get_or_create_user(
    user_data: DiscordUser,
    ctx: &ApiContext
) -> ApiResult<Uuid> {
    let user_meta = sqlx::query!(
        r#"select user_id from users where discord_id = $1"#,
        user_data.id
    )
    .fetch_optional(&ctx.pool)
    .await?;

    if let Some(user_meta) = user_meta {
        return Ok(user_meta.user_id)
    } 

    let permissions = Permissions::default().bits() as i32;
    
    let record = sqlx::query!(
        r#"
        insert into users
            (username, email, discord_id, permissions)
        values 
            ($1, $2, $3, $4)
        on conflict (email) do update
            set discord_id = $3
        returning 
            user_id
        "#,
        user_data.username,
        user_data.email,
        user_data.id,
        permissions
    )
    .fetch_one(&ctx.pool)
    .await?;

    Ok(record.user_id)
}
