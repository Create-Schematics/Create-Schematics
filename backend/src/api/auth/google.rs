use clap::Args;
use oauth2::{ClientSecret, ClientId, AuthUrl, TokenUrl, RedirectUrl};
use oauth2::basic::BasicClient;

#[derive(Args, Debug)]
pub struct GoogleOauthArguments {  
    #[arg(help = "Your google oauth client id")]
    #[arg(env = "GOOGLE_CLIENT_ID", long = "google-client-id")]
    pub google_client_id: String,

    #[arg(help = "Your google oauth client secret")]
    #[arg(env = "GOOGLE_CLIENT_SECRET", long = "google-client-secret")]
    pub google_client_secret: String,
}

pub (in crate::api::auth) fn build_client(
    GoogleOauthArguments {
        google_client_id,
        google_client_secret,
        ..
    }: GoogleOauthArguments
) -> Result<BasicClient, anyhow::Error> {
    let self_address = dotenv::var("SELF_ADDRESS")?;

    let google_client_secret = ClientSecret::new(google_client_secret);
    let google_client_id = ClientId::new(google_client_id);

    let auth_url = AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string())?;
    let token_url = TokenUrl::new("https://www.googleapis.com/oauth2/v4/token".to_string())?;

    let redirect_url = RedirectUrl::new(format!("{self_address}/api/auth/google/callback"))?;

    let client = BasicClient::new(
        google_client_id,
        Some(google_client_secret),
        auth_url,
        Some(token_url)
    )
    .set_redirect_uri(redirect_url);

    Ok(client)
}  