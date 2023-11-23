use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Serialize, ToSchema)]
pub struct User {
    pub user_id: Uuid,
    
    pub username: String,

    pub email: String,

    #[serde(skip_serializing)]
    pub password_hash: String
}

bitflags::bitflags! {
    #[derive(Serialize, Deserialize, ToSchema)]
    #[serde(transparent)]
    pub struct Permissions: u64 {
        const VOTE = 1 << 0;
        const COMMENT = 1 << 1;
        const POST = 1 << 2;
        const MODERATE_COMMENTS = 1 << 3;
        const MODERATE_POSTS = 1 << 4;
        const MODERATE_USERS = 1 << 5;
    }
}   

impl From<i64> for Permissions {
    fn from(value: i64) -> Self {
        Permissions::from_bits(value as u64).unwrap_or_default()
    }
}

impl Default for Permissions {
    fn default() -> Self {
        Permissions::VOTE | Permissions::COMMENT | Permissions::POST
    }
}