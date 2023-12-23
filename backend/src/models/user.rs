use poem_openapi_derive::{Enum, Object};
use uuid::Uuid;

#[derive(Debug, Object)]
pub struct User {
    pub user_id: Uuid,
    pub username: String,
    pub displayname: String,
    pub avatar: Option<String>,
    pub about: Option<String>,
    pub role: Role,
}

impl User {
    pub fn is_moderator(&self) -> bool {
        self.role.is_moderator()
    }

    pub fn is_administrator(&self) -> bool {
        self.role.is_administrator()    
    }
}

#[derive(Enum, Serialize, Debug)]
#[serde(rename_all="snake_case")]
#[non_exhaustive]
pub enum Role {
    User,
    Moderator,
    Administrator
}

impl From<std::string::String> for Role {
    fn from(value: std::string::String) -> Self {
        match value.as_str() {
            "administrator" => Self::Administrator,
            "moderator" => Self::Moderator,
            _ => Self::User,
        }
    }
}

impl Role {
    pub fn is_moderator(&self) -> bool {
        match self {
            Self::Moderator | Self::Administrator => true,
            _ => false,
        }
    }

    pub fn is_administrator(&self) -> bool {
        match self {
            Self::Administrator => true,
            _ => false,
        }
    }
}