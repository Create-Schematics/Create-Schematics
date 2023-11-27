use sqlx::Postgres;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::error::ApiError;
use crate::response::ApiResult;

use super::user::{User, Permissions};

#[derive(Debug, Serialize, ToSchema)]
pub struct Schematic {
    pub schematic_id: String,
    pub schematic_name: String,
    pub game_version_id: i32,
    pub create_version_id: i32,
    pub author: Uuid,
    pub downloads: i64,
}

impl Schematic { 
    pub async fn check_user_permissions<'a, E> (
        user: User,
        schematic_id: &str,
        permissions: Permissions,
        executor: E
    ) -> ApiResult<()>
    where
        E: sqlx::Executor<'a, Database = Postgres>,
    {
        if user.permissions.contains(permissions) {
            return Ok(());
        }

        let schematic_meta = sqlx::query!(
            r#"select author from schematics where schematic_id = $1"#,
            schematic_id
        )
        .fetch_optional(executor)
        .await?
        .ok_or(ApiError::NotFound)?;

        if schematic_meta.author == user.user_id {
            Ok(())
        } else {
            Err(ApiError::Forbidden)
        }
    }
}