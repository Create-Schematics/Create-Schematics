use poem_openapi_derive::Object;
use uuid::Uuid;

#[derive(Debug, Serialize, Object)]
pub struct Comment {
    pub comment_id: Uuid,
    pub comment_author: Uuid,
    pub comment_body: String,
    pub schematic_id: String
}
