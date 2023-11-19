use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Serialize, ToSchema)]
pub struct Schematic {
    pub schematic_id: i64,
    pub schematic_name: String,
    pub game_version: i32,
    pub create_version: i32,
    pub author: Uuid,
    pub downloads: i64,
}