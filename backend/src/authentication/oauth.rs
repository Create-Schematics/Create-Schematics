use std::env;

use oauth2::{ClientId, ClientSecret, AuthUrl, TokenUrl, RedirectUrl, Scope};
use oauth2::basic::BasicClient;
use poem_openapi_derive::Enum;
use strum::Display;

#[cfg(feature="discord-oauth")]
use crate::api::auth::discord::DiscordUser;

#[cfg(feature="github-oauth")]
use crate::api::auth::github::GitHubUser;

#[cfg(feature="google-oauth")]
use crate::api::auth::google::GoogleUser;

#[cfg(feature="microsoft-oauth")]
use crate::api::auth::microsoft::MicrosoftUser;
use crate::api::auth::modrinth::ModrinthUser;

#[derive(Deserialize, Serialize, Display, Debug, Enum)]
#[oai(rename_all="lowercase")]
#[serde(rename_all="lowercase")]
#[strum(serialize_all="lowercase")]
pub enum OauthProvider {
    #[cfg(feature="github-oauth")]
    GitHub,

    #[cfg(feature="microsoft-oauth")]
    Microsoft,

    #[cfg(feature="google-oauth")]
    Google,
    
    #[cfg(feature="discord-oauth")]
    Discord,

    #[cfg(feature="modrinth-oauth")]
    Modrinth
}

pub struct OauthUser {
    pub oauth_id: String,
    pub username: String,
    pub display_name: Option<String>,
    pub email: Option<String>,
    pub avatar_url: Option<String>,
}

#[derive(Clone)]
pub struct ScopedClient {
    pub inner: BasicClient,
    pub scopes: Vec<Scope>,
    pub data_uri: String,
    pub extractor: UserExtractor
}

impl ScopedClient {
    pub fn extract_user(&self, response: &[u8]) -> Result<OauthUser, anyhow::Error> {
        (self.extractor)(response).map_err(anyhow::Error::new)
    }
}

struct ClientConfig<'a> {
    redirect_uri: &'a str,
    client_id_env: &'a str,
    client_secret_env: &'a str,
    auth_url: &'a str,
    token_url: &'a str,
    scopes: &'a [&'a str],
    data_uri: &'a str,
    extractor: UserExtractor
}

type UserExtractor = fn(response: &[u8]) -> Result<OauthUser, serde_json::Error>;

impl OauthProvider {
    pub fn build_client(&self) -> Result<ScopedClient, anyhow::Error> {
        let config = match self {
            #[cfg(feature="github-oauth")]
            OauthProvider::GitHub => ClientConfig {
                redirect_uri: "/api/auth/github/callback",
                client_id_env: "GITHUB_CLIENT_ID",
                client_secret_env: "GITHUB_CLIENT_SECRET",
                auth_url: "https://github.com/login/oauth/authorize?response_type=code",
                token_url: "https://github.com/login/oauth/access_token",
                scopes: &[
                    "read:user", 
                    "user:email"
                ],
                data_uri: "https://api.github.com/user",
                extractor: |r| serde_json::from_slice::<GitHubUser>(&r).map(|u| u.into())
            },
            
            #[cfg(feature="microsoft-oauth")]
            OauthProvider::Microsoft => ClientConfig {
                redirect_uri: "/api/auth/microsoft/callback",
                client_id_env: "MICROSOFT_CLIENT_ID",
                client_secret_env: "MICROSOFT_CLIENT_SECRET",
                auth_url: "https://login.microsoftonline.com/common/oauth2/v2.0/authorize",
                token_url: "https://login.microsoftonline.com/common/oauth2/v2.0/token",
                scopes: &[
                    "user.read"
                ],
                data_uri: "https://graph.microsoft.com/v1.0/me?$select=id,displayName,mail,userPrincipalName",
                extractor: |r| serde_json::from_slice::<MicrosoftUser>(&r).map(|u| u.into())
            },

            #[cfg(feature="google-oauth")]
            OauthProvider::Google => ClientConfig {
                redirect_uri: "/api/auth/google/callback",
                client_id_env: "GOOGLE_CLIENT_ID",
                client_secret_env: "GOOGLE_CLIENT_SECRET",
                auth_url: "https://accounts.google.com/o/oauth2/v2/auth",
                token_url: "https://www.googleapis.com/oauth2/v4/token",
                scopes: &[
                    "openid",
                    "email",
                    "profile"
                ],
                data_uri: "https://www.googleapis.com/oauth2/v3/userinfo",
                extractor: |r| serde_json::from_slice::<GoogleUser>(&r).map(|u| u.into())
            },

            #[cfg(feature="discord-oauth")]
            OauthProvider::Discord => ClientConfig {
                redirect_uri: "/api/auth/discord/callback",
                client_id_env: "DISCORD_CLIENT_ID",
                client_secret_env: "DISCORD_CLIENT_SECRET",
                auth_url: "https://discord.com/api/oauth2",
                token_url: "https://discord.com/api/oauth2/token",
                scopes: &[
                    "identify",
                    "email"
                ],
                data_uri: "https://discordapp.com/api/users/@me",
                extractor: |r| serde_json::from_slice::<DiscordUser>(&r).map(|u| u.into())
            },
            
            #[cfg(feature="modrinth-oauth")]
            OauthProvider::Modrinth => ClientConfig {
                redirect_uri: "/api/auth/modrinth/callback",
                client_id_env: "MODRINTH_CLIENT_ID",
                client_secret_env: "MODRINTH_CLIENT_SECRET",
                auth_url: "https://modrinth.com/auth/authorize",
                token_url: "https://api.modrinth.com/_internal/oauth/token", // Supposedly _internal will be later replaced with `v3`
                scopes: &[
                    "USER_READ",
                    "USER_READ_EMAIL"
                ],
                data_uri: "https://api.modrinth.com/v2/user",
                extractor: |r| serde_json::from_slice::<ModrinthUser>(&r).map(|u| u.into()),
            }
        };

        config.try_into()
    }
}

impl<'a> TryFrom<ClientConfig<'a>> for ScopedClient {
    type Error = anyhow::Error;

    fn try_from(cfg: ClientConfig<'a>) -> Result<Self, Self::Error> {
        let self_address = env::var("SELF_ADDRESS")?;

        let client_id = ClientId::new(env::var(cfg.client_id_env)?);
        let client_secret = ClientSecret::new(env::var(cfg.client_secret_env)?);
        
        let auth_url = AuthUrl::new(cfg.auth_url.to_string())?;
        let token_url = TokenUrl::new(cfg.token_url.to_string())?;

        let callback = cfg.redirect_uri.to_string();
        let redirect_url = RedirectUrl::new(format!("{self_address}{callback}"))?;

        let inner = BasicClient::new(
            client_id, 
            Some(client_secret), 
            auth_url, 
            Some(token_url)
        )
        .set_redirect_uri(redirect_url);

        Ok(Self {
            inner,
            scopes: cfg.scopes.iter().map(|s| Scope::new(s.to_string())).collect(),
            data_uri: cfg.data_uri.to_string(),
            extractor: cfg.extractor
        })
    }
}