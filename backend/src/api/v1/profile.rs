use axum::{Router, routing::get, Json, extract::{State, Path, Query}};
use uuid::Uuid;

use crate::{api::ApiContext, response::ApiResult, models::schematic::Schematic};

use super::comments::PaginationQuery;

pub (in crate::api::v1) fn configure() -> Router<ApiContext> {
    Router::new()
        .route(
            "/users/:id/schematics",
            get(get_uploaded_schematics)
        )
}

#[utoipa::path(
    get,
    path = "/users/{id}/schematics",
    context_path = "/api/v1",
    tag = "v1",
    params(
        ("id" = String, Path, description = "The id of the user to get the schematics from"),
        ("query" = PaginationQuery, Query, description = "How many schematics to fetch")
    ),
    responses(
        (status = 200, description = "Successfully retrieved the schematics", body = [Schematic], content_type = "application/json"),
        (status = 404, description = "A schematic with that id was not found"),
        (status = 500, description = "An internal server error occurred")
    ),
    security(())
)]
async fn get_uploaded_schematics(
    State(ctx): State<ApiContext>,
    Path(user_id): Path<Uuid>,
    Query(query): Query<PaginationQuery>
) -> ApiResult<Json<Vec<Schematic>>> {
    let schematics = sqlx::query_as!(
        Schematic,
        r#"
        select schematic_id, schematic_name,
               game_version_id, create_version_id,
               downloads, author
        from schematics
        where author = $1
        limit $2 offset $3
        "#,
        user_id,
        query.limit.unwrap_or(20),
        query.offset.unwrap_or(0)
    )
    .fetch_all(&ctx.pool)
    .await?;

    Ok(Json(schematics))
}