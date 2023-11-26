use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Serialize, ToSchema)]
pub struct Comment {
    pub comment_id: String,
    pub comment_author: Uuid,
    pub comment_body: String,
    pub schematic_id: String
}