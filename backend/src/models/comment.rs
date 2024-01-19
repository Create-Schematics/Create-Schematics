use poem_openapi_derive::Object;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, Serialize, Object)]
pub struct Comment {
    pub comment_id: Uuid,
    pub parent: Option<Uuid>,
    pub comment_author: Uuid,
    pub comment_body: String,
    pub schematic_id: String,
    pub updated_at: Option<OffsetDateTime>,
    pub created_at: OffsetDateTime
}
