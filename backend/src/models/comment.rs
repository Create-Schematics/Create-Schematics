use poem_openapi_derive::Object;
use sqlx::Postgres;
use uuid::Uuid;

use crate::{response::ApiResult, error::ApiError};

use super::user::{User, Permissions};

#[derive(Debug, Serialize, Object)]
pub struct Comment {
    pub comment_id: Uuid,
    pub comment_author: Uuid,
    pub comment_body: String,
    pub schematic_id: String
}

impl Comment {
    pub async fn check_user_permissions<'a, E> (
        user: User,
        comment_id: &Uuid,
        permissions: Permissions,
        executor: E
    ) -> ApiResult<()>
    where
        E: sqlx::Executor<'a, Database = Postgres>,
    {
        if user.permissions.contains(permissions) {
            return Ok(());
        }

        let comment_meta = sqlx::query!(
            r#"select comment_author from comments where comment_id = $1"#,
            comment_id
        )
        .fetch_optional(executor)
        .await?
        .ok_or(ApiError::NotFound)?;

        if comment_meta.comment_author == user.user_id {
            Ok(())
        } else {
            Err(ApiError::Forbidden)
        }
    }
}