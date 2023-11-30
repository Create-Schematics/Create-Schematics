use axum::{Router, extract::{State, Path, Query}, routing::get, Json};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{api::ApiContext, response::ApiResult, authentication::session::Session, error::ApiError};

use super::comments::PaginationQuery;

pub (in crate::api::v1) fn configure() -> Router<ApiContext> {
    Router::new()
        .route(
            "/tags", 
            get(get_valid_tags)
        )
        .route(
            "/schematics/:id/tags",
             get(get_schematic_tags)
             .post(tag_schematic_by_id)
             .delete(untag_schematic_by_id)
        )
}

#[derive(Debug, Deserialize, ToSchema)]
pub (in crate::api) struct Tags {
    tag_names: Vec<i64>,
}

#[derive(Debug, Serialize, ToSchema)]
pub (in crate::api) struct FullTag {
    tag_id: i64,
    tag_name: String,
}

#[utoipa::path(
    get,
    path = "/schematics/{id}/tags",
    context_path = "/api/v1",
    tag = "v1",
    params(
        ("id" = String, Path, description = "The id of the schematic to fetch tags from")
    ),
    responses(
        (status = 200, description = "Successfully retrieved the schematic's tags", body = [FullTag], content_type = "application/json"),
        (status = 404, description = "A schematic with that id was not found"),
        (status = 500, description = "An internal server error occurred")
    ),
    security(())
)]
async fn get_schematic_tags(
    State(ctx): State<ApiContext>,
    Path(schematic_id): Path<Uuid>
) -> ApiResult<Json<Vec<FullTag>>> {
    let tags = sqlx::query_as!(
        FullTag,
        r#"
        select tag_id, tag_name
        from applied_tags
        inner join tags using (tag_id)
        where schematic_id = $1 
        "#,
        schematic_id
    )
    .fetch_all(&ctx.pool)
    .await?;

    Ok(Json(tags))
}

#[utoipa::path(
    get,
    path = "/tags",
    context_path = "/api/v1",
    tag = "v1",
    params(
        ("query" = PaginationQuery, Query, description = "How many tags to fetch")
    ),
    responses(
        (status = 200, description = "Successfully retrieved the tags", body = [FullTag], content_type = "application/json"),
        (status = 500, description = "An internal server error occurred")
    ),
    security(())
)]
async fn get_valid_tags(
    State(ctx): State<ApiContext>,
    Query(query): Query<PaginationQuery>
) -> ApiResult<Json<Vec<FullTag>>> {
    let tags = sqlx::query_as!(
        FullTag,
        r#"
        select tag_id, tag_name
        from tags
        limit $1 offset $2
        "#,
        query.limit.unwrap_or(20),
        query.offset.unwrap_or(0)
    )
    .fetch_all(&ctx.pool)
    .await?;

    Ok(Json(tags))
}

#[utoipa::path(
    post,
    path = "/schematics/{id}/tags",
    context_path = "/api/v1",
    tag = "v1",
    params(
        ("id" = String, Path, description = "The id of the schematic to remove")
    ),
    request_body(
        content = TagBody, description = "The new tags to apply by name", content_type = "application/json"
    ),
    responses(
        (status = 200, description = "Successfully added tags to schematic", body = Schematic, content_type = "application/json"),
        (status = 401, description = "You must be logged in to tag a schematic"),
        (status = 403, description = "You do not have permission to tag this schematic"),
        (status = 500, description = "An error occurred while tagging the schematic")
    ),
    security(("session_cookie" = []))
)]
async fn tag_schematic_by_id(
    State(ctx): State<ApiContext>,
    session: Session,
    Path(schematic_id): Path<Uuid>,
    Json(query): Json<Tags>
) -> ApiResult<()> {
    let schematic_meta = sqlx::query!(
        r#"select author from schematics where schematic_id = $1"#,
        schematic_id
    )
    .fetch_optional(&ctx.pool)
    .await?
    .ok_or(ApiError::NotFound)?;

    if schematic_meta.author != session.user_id {
        return Err(ApiError::Forbidden.into());
    }

    sqlx::query!(
        // Unfortunately sqlx does not inserting multiple records 
        // directly without using a query builder which would mean 
        // loosing out on compiler checking. None of this is ideal
        // if you have a better solution please let me know.
        // 
        // see: https://github.com/launchbadge/sqlx/issues/294
        r#"
        insert into applied_tags (
            schematic_id, tag_id
        )
        select $1, tag_id
        from unnest($2::bigint[]) as tag_id
        on conflict do nothing
        "#,
        schematic_id,
        &query.tag_names[..],
    )
    .execute(&ctx.pool)
    .await?;

    Ok(())
}

#[utoipa::path(
    delete,
    path = "/schematics/{id}/tags",
    context_path = "/api/v1",
    tag = "v1",
    params(
        ("id" = String, Path, description = "The id of the schematic to remove tags from")
    ),
    request_body(
        content = Tags, description = "The tags to remove from the schematic", content_type = "application/json"
    ),
    responses(
        (status = 200, description = "Successfully removed the tags from the schematic"),
        (status = 401, description = "You need to be logged in to delete tags from a schematic"),
        (status = 403, description = "You do not have permission to delete tags from this schematic"),
        (status = 404, description = "A schematic with that id was not found"),
        (status = 500, description = "An internal server error occurred")
    ),
    security(("session_cookie" = []))
)]
async fn untag_schematic_by_id(
    State(ctx): State<ApiContext>,
    session: Session,
    Path(schematic_id): Path<Uuid>,
    Json(query): Json<Tags>
) -> ApiResult<()> {
    let schematic_meta = sqlx::query!(
        r#"select author from schematics where schematic_id = $1"#,
        schematic_id
    )
    .fetch_optional(&ctx.pool)
    .await?
    .ok_or(ApiError::NotFound)?;

    if schematic_meta.author != session.user_id {
        return Err(ApiError::Forbidden.into());
    }

    sqlx::query!(
        r#"
        delete from applied_tags 
        where schematic_id = $1 
        and tag_id = ANY($2)
        "#,
        schematic_id,
        &query.tag_names
    )
    .execute(&ctx.pool)
    .await?;

    Ok(())
}