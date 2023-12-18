use crate::authentication::oauth::OauthUser;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MicrosoftUser {
    pub id: String,
    pub display_name: Option<String>,
    pub mail: String,
    pub user_principal_name: String,
}

impl From<MicrosoftUser> for OauthUser {
    fn from(microsoft_user: MicrosoftUser) -> Self {
        Self {
            oauth_id: microsoft_user.id,
            username: username_from_email(&microsoft_user.mail),
            display_name: microsoft_user.display_name,
            email: Some(microsoft_user.mail),
            avatar_url: None,
        }
    } 
}

fn username_from_email(email: &str) -> String {
    email.split('@')
        .next()
        .unwrap_or_default()
        .to_string()
}