use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Serialize, ToSchema)]
pub struct Schematic {
    pub schematic_id: String,
    pub schematic_name: String,
    pub game_version_id: i32,
    pub create_version_id: i32,
    pub author: Uuid,
    pub downloads: i64,
}