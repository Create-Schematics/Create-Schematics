use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Serialize, ToSchema)]
pub struct User {
    pub user_id: Uuid,

    #[schema(example="My username")]
    #[schema(min_length=3, max_length=20)]
    pub username: String,

    #[schema(value_type=u64, example=7)]
    pub permissions: Permissions,

    #[schema(example="email@email.com")]
    pub email: String,

    #[serde(skip_serializing)]
    pub password_hash: String
}

bitflags::bitflags! {
    #[derive(Debug, Serialize, Deserialize, ToSchema)]
    #[serde(transparent)]
    pub struct Permissions: u32 {
        const VOTE = 1 << 0;
        const COMMENT = 1 << 1;
        const POST = 1 << 2;
        const MODERATE_COMMENTS = 1 << 3;
        const MODERATE_POSTS = 1 << 4;
        const MODERATE_USERS = 1 << 5;
    }
}   

impl From<i32> for Permissions {
    fn from(value: i32) -> Self {
        Permissions::from_bits(value as u32).unwrap_or_default()
    }
}

impl Default for Permissions {
    fn default() -> Self {
        Permissions::VOTE | Permissions::COMMENT | Permissions::POST
    }
}