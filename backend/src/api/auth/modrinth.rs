use crate::authentication::oauth::OauthUser;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct ModrinthUser {
    pub id: String,
    pub username: String,
    pub name: String,
    pub email: String,
    pub avatar_url: String
}

impl From<ModrinthUser> for OauthUser {
    fn from(modrinth_user: ModrinthUser) -> Self {
        Self {
            username: modrinth_user.username,
            display_name: Some(modrinth_user.name),
            email: Some(modrinth_user.email),
            avatar_url: Some(modrinth_user.avatar_url),
            oauth_id: modrinth_user.id,
        }
    }
}

