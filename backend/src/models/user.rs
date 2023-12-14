use poem_openapi_derive::Object;
use uuid::Uuid;

#[derive(Debug, Object)]
pub struct User {
    pub user_id: Uuid,
    pub username: String,
    pub avatar: Option<String>,
    pub about: Option<String>,
    pub permissions: Permissions,
}

bitflags::bitflags! {
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(transparent)]
    pub struct Permissions: u32 {
        const MODERATE_COMMENTS = 1 << 1;
        const MODERATE_POSTS = 1 << 2;
        const MODERATE_USERS = 1 << 3;
    }
}   

impl From<i32> for Permissions {
    fn from(value: i32) -> Self {
        Permissions::from_bits(value as u32).unwrap_or_default()
    }
}

impl Default for Permissions {
    fn default() -> Self {
        Permissions::empty()
    }
}