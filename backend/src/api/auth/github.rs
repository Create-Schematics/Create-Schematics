use clap::Args;
use oauth2::{ClientSecret, ClientId, AuthUrl, TokenUrl, RedirectUrl};
use oauth2::basic::BasicClient;

#[derive(Args, Debug)]
pub struct GitHubOauthArguments {  
    #[arg(help = "Your github oauth client id")]
    #[arg(env = "GITHUB_CLIENT_ID", long = "github-client-id")]
    pub github_client_id: String,

    #[arg(help = "Your github oauth client secret")]
    #[arg(env = "GITHUB_CLIENT_SECRET", long = "github-client-secret")]
    pub github_client_secret: String,
}

pub (in crate::api::auth) fn build_client(
    GitHubOauthArguments {
        github_client_id,
        github_client_secret,
        ..
    }: GitHubOauthArguments
) -> Result<BasicClient, anyhow::Error> {
    let self_address = dotenv::var("SELF_ADDRESS")?;

    let github_client_secret = ClientSecret::new(github_client_secret);
    let github_client_id = ClientId::new(github_client_id);

    let auth_url = AuthUrl::new("https://github.com/login/oauth/authorize?response_type=code".to_string())?;
    let token_url = TokenUrl::new("https://github.com/login/oauth/access_token".to_string())?;

    let redirect_url = RedirectUrl::new(format!("{self_address}/api/auth/github/callback"))?;

    let client = BasicClient::new(
        github_client_id,
        Some(github_client_secret),
        auth_url,
        Some(token_url)
    )
    .set_redirect_uri(redirect_url);

    Ok(client)
}  
