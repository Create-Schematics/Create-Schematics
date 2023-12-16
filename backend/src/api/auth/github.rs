use crate::authentication::oauth::OauthUser;

#[derive(Serialize, Deserialize, Debug)]
pub struct GitHubUser {
    pub login: String,
    pub id: u64,
    pub avatar_url: String,
    pub name: Option<String>,
    pub email: Option<String>,
}

impl From<GitHubUser> for OauthUser {
    fn from(github_user: GitHubUser) -> Self {
        Self {
            oauth_id: github_user.id.to_string(),
            username: github_user.login,
            display_name: github_user.name,
            email: github_user.email,
            avatar_url: Some(github_user.avatar_url),
        }
    }
}