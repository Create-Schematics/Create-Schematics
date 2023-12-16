use crate::authentication::oauth::OauthUser;

#[derive(Serialize, Deserialize, Debug)]
pub struct DiscordUser {
    pub id: String,
    pub username: String,
    pub avatar: Option<String>,
    pub global_name: Option<String>,
    pub email: String,
}

impl From<DiscordUser> for OauthUser {
    fn from(discord_user: DiscordUser) -> Self {
        let avatar_url = discord_user
            .avatar
            .map(|x| format!("https://cdn.discordapp.com/avatars/{}/{}.webp", discord_user.id, x));

        Self {
            oauth_id: discord_user.id,
            username: discord_user.username,
            display_name: discord_user.global_name,
            email: Some(discord_user.email),
            avatar_url,
        }
    }
}