use axum::extract::{State, Query};
use axum::response::Redirect;
use axum::routing::get;
use axum::{Router, Extension};
use clap::Args;
use oauth2::{ClientId, ClientSecret, AuthUrl, TokenUrl, RedirectUrl, RevocationUrl, CsrfToken, Scope};
use oauth2::basic::BasicClient;

use crate::api::ApiContext;
use crate::response::ApiResult;

#[derive(Args, Debug)]
pub struct GoogleOauthArguments {
    #[arg(help = "Your google oauth client id")]
    #[arg(env = "GOOGLE_CLIENT_ID", long = "google-client-id")]
    pub google_client_id: String,

    #[arg(help = "Your google oauth client secret")]
    #[arg(env = "GOOGLE_CLIENT_SECRET", long = "google-client-secret")]
    pub google_client_secret: String,
}

pub (in crate::api::auth) fn configure(
    args: GoogleOauthArguments
) -> Result<Router<ApiContext>, anyhow::Error> {
    let oauth_client = build_client(args)?;
    
    let router = Router::new()
        .route("/auth/google", get(google_authorization))
        .route("/auth/google_callback", get(google_callback))
        .layer(Extension(oauth_client));

    Ok(router)
}

fn build_client(
    GoogleOauthArguments {
        google_client_id,
        google_client_secret,
        ..
    }: GoogleOauthArguments
) -> Result<BasicClient, anyhow::Error> {
    let google_client_secret = ClientSecret::new(google_client_secret);
    let google_client_id = ClientId::new(google_client_id);

    let auth_url = AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string())?;
    let token_url = TokenUrl::new("https://www.googleapis.com/oauth2/v3/token".to_string())?;

    let redirect_url = RedirectUrl::new("http://localhost:3000/api/auth/google_callback".to_string())?;
    let revocation_url = RevocationUrl::new("https://oauth2.googleapis.com/revoke".to_string())?;

    let client = BasicClient::new(
        google_client_id,
        Some(google_client_secret),
        auth_url,
        Some(token_url)
    )
    .set_redirect_uri(redirect_url)
    .set_revocation_uri(revocation_url);

    Ok(client)
}   

#[derive(Debug, Deserialize)]
pub (in crate::api) struct AuthRequest {
    pub code: String,
    pub state: String
}

async fn google_authorization(
    Extension(oauth_client): Extension<BasicClient>
) -> Redirect {
    let (auth_url, _csrf_token) = oauth_client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("profile".to_string()))
        .add_scope(Scope::new("email".to_string()))
        .url();

    Redirect::to(&auth_url.to_string())
}

async fn google_callback(
    State(ctx): State<ApiContext>,
    Query(query): Query<AuthRequest>,
    Extension(oauth_client): Extension<BasicClient>
) -> ApiResult<()> {
    unimplemented!()
}