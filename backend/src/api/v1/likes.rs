use axum::extract::{State, Path, Query};
use axum::routing::post;
use utoipa::ToSchema;
use axum::Router;
use uuid::Uuid;

use crate::error::ApiError;
use crate::response::ApiResult;
use crate::authentication::session::Session;
use crate::api::ApiContext;

pub (in crate::api::v1) fn configure() -> Router<ApiContext> {
    Router::new()
        .route(
            "/schematics/:id/like", 
            post(like_schematic)
            .delete(remove_like_from_schematic)
        )
}

#[derive(Deserialize, Debug, ToSchema)]
pub (in crate::api) struct LikeQuery {
    action: Option<LikeAction> 
}

#[derive(Deserialize, Debug, ToSchema)]
pub (in crate::api) enum LikeAction {
    #[serde(rename = "like")]
    Like,

    #[serde(rename = "dislike")]
    Dislike
}

impl LikeAction {
    #[inline]
    pub fn positive(self) -> bool { 
        matches!(self, Self::Like)
    }
}

#[utoipa::path(
    delete,
    path = "/schematics/{id}/like",
    context_path = "/api/v1",
    tag = "v1",
    params(
        ("id" = String, Path, description = "The id of the schematic to vote on"),
        ("action" = String, Query, description = "Weather you want to like, or dislike the schematic")
    ),
    responses(
        (status = 200, description = "Successfully liked the schematic"),
        (status = 401, description = "You need to be logged in to like a schematic"),
        (status = 404, description = "Could not find a schematic with that id"),
        (status = 500, description = "An internal server error occurred")
    ),
    security(("session_cookie" = []))
)]
async fn like_schematic(
    State(ctx): State<ApiContext>,
    session: Session,
    Path(schematic_id): Path<Uuid>,
    Query(query): Query<LikeQuery>
) -> ApiResult<()> {
    let action = query.action.ok_or(ApiError::BadRequest)?;

    sqlx::query!(
        r#"
        insert into schematic_likes (
            schematic_id, user_id, 
            positive
        )
        values (
            $1, $2, $3
        )
        on conflict (
            schematic_id, user_id
        )
        do update set positive = $3
        "#,
        schematic_id,
        session.user_id,
        action.positive()
    )
    .execute(&ctx.pool)
    .await?;

    Ok(())
}

#[utoipa::path(
    delete,
    path = "/schematics/{id}/like",
    context_path = "/api/v1",
    tag = "v1",
    params(
        ("id" = String, Path, description = "The id of the comment to remove the vote from")
    ),
    responses(
        (status = 200, description = "Successfully removed the vote"),
        (status = 401, description = "You need to be logged in to remove al ike"),
        (status = 404, description = "You have not liked this schematic"),
        (status = 500, description = "An internal server error occurred")
    ),
    security(("session_cookie" = []))
)]
async fn remove_like_from_schematic(
    State(ctx): State<ApiContext>,
    session: Session,
    Path(schematic_id): Path<Uuid>,
) -> ApiResult<()> {
    let mut transaction = ctx.pool.begin().await?;

    let result = sqlx::query!(
        r#"
        with deleted_like as (
            delete from schematic_likes
            where user_id = $1
            and schematic_id = $2
            returning 1
        )
        select
            exists(select 1 from schematic_likes where schematic_id = $2) "existed",
            exists(select 1 from deleted_like) "deleted"
        "#,
        session.user_id,
        schematic_id
    )
    .fetch_one(&mut *transaction)
    .await?;

    transaction.commit().await?;

    if result.deleted.unwrap_or_default() {
        Ok(())
    } else if result.existed.unwrap_or_default() {
        Err(ApiError::Forbidden)
    } else {
        Err(ApiError::NotFound)
    }
}
