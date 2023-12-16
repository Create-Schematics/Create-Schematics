use crate::authentication::oauth::OauthUser;

#[derive(Deserialize, Debug)]
pub struct GoogleUser {
    pub id: String,
    pub email: String,
    pub name: Option<String>,
    pub picture: Option<String>,
}

impl From<GoogleUser> for OauthUser {
    fn from(google_user: GoogleUser) -> Self {
        let username = google_user
            .name
            .unwrap_or(username_from_email(&google_user.email));

        Self {
            oauth_id: google_user.id,
            username,
            display_name: google_user.name,
            email: Some(google_user.email),
            avatar_url: google_user.picture,
        }
    }
}

fn username_from_email(email: &str) -> String {
    email.split('@')
        .next()
        .unwrap_or_default()
        .to_string()
}