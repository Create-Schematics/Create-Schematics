use axum::Router;
use clap::Args;

use self::discord::DiscordOauthArguments;
use self::google::GoogleOauthArguments;

use super::ApiContext;

pub mod google;
pub mod discord;

#[derive(Args, Debug)]
pub struct StartCommandOauthArguments {
    #[command(next_help_heading = "Google")]
    #[command(flatten)]
    pub google: GoogleOauthArguments,

    #[command(next_help_heading = "Discord")]
    #[command(flatten)]
    pub discord: DiscordOauthArguments
}

pub (in crate::api) fn configure(
    StartCommandOauthArguments {    
        google,
        discord,
        ..
    }: StartCommandOauthArguments
) -> Result<Router<ApiContext>, anyhow::Error> {
    let router = Router::new()
        .merge(google::configure(google)?)
        .merge(discord::configure(discord)?);

    Ok(router)
}