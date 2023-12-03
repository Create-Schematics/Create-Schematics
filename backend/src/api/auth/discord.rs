use clap::Args;
use oauth2::{ClientSecret, ClientId, AuthUrl, TokenUrl, RedirectUrl};
use oauth2::basic::BasicClient;

#[derive(Args, Debug)]
pub struct DiscordOauthArguments {  
    #[arg(help = "Your discord oauth client id")]
    #[arg(env = "DISCORD_CLIENT_ID", long = "discord-client-id")]
    pub discord_client_id: String,

    #[arg(help = "Your discord oauth client secret")]
    #[arg(env = "DISCORD_CLIENT_SECRET", long = "discord-client-secret")]
    pub discord_client_secret: String,
}

pub (in crate::api::auth) fn build_client(
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

    let redirect_url = RedirectUrl::new(format!("{self_address}/api/auth/discord/callback"))?;

    let client = BasicClient::new(
        discord_client_id,
        Some(discord_client_secret),
        auth_url,
        Some(token_url)
    )
    .set_redirect_uri(redirect_url);

    Ok(client)
}  
