use poem_openapi_derive::Object;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, Serialize, Object)]
pub struct Schematic {
    pub schematic_id: Uuid,
    pub body: String,
    pub schematic_name: String,
    pub game_version_id: i32,
    pub create_version_id: i32,
    pub author: Uuid,
    pub images: Vec<String>,
    pub downloads: i64,
    pub updated_at: Option<OffsetDateTime>,
    pub created_at: OffsetDateTime
}
