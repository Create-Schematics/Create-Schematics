use clap::Args;
use oauth2::{ClientSecret, ClientId, AuthUrl, TokenUrl, RedirectUrl};
use oauth2::basic::BasicClient;

#[derive(Args, Debug)]
pub struct MicrosoftOauthArguments {  
    #[arg(help = "Your microsoft oauth client id")]
    #[arg(env = "MICROSOFT_CLIENT_ID", long = "microsoft-client-id")]
    pub microsoft_client_id: String,

    #[arg(help = "Your microsoft oauth client secret")]
    #[arg(env = "MICROSOFT_CLIENT_SECRET", long = "microsoft-client-secret")]
    pub microsoft_client_secret: String,
}

pub (in crate::api::auth) fn build_client(
    MicrosoftOauthArguments {
        microsoft_client_id,
        microsoft_client_secret,
        ..
    }: MicrosoftOauthArguments
) -> Result<BasicClient, anyhow::Error> {
    let self_address = dotenv::var("SELF_ADDRESS")?;

    let microsoft_client_secret = ClientSecret::new(microsoft_client_secret);
    let microsoft_client_id = ClientId::new(microsoft_client_id);

    let auth_url = AuthUrl::new("https://login.microsoftonline.com/common/oauth2/v2.0/authorize?response_type=code".to_string())?;
    let token_url = TokenUrl::new("https://login.microsoftonline.com/common/oauth2/v2.0/token".to_string())?;

    let redirect_url = RedirectUrl::new(format!("{self_address}/api/auth/microsoft/callback"))?;

    let client = BasicClient::new(
        microsoft_client_id,
        Some(microsoft_client_secret),
        auth_url,
        Some(token_url)
    )
    .set_redirect_uri(redirect_url);

    Ok(client)
}  
