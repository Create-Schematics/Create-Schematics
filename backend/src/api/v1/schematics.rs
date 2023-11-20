use axum::routing::get;
use axum::{Router, Json};
use axum::extract::{State, Path, Query};
use utoipa::ToSchema;

use crate::authentication::session::Session;
use crate::error::ApiError;
use crate::response::ApiResult;
use crate::models::schematic::Schematic;
use crate::api::ApiContext;

#[derive(Debug, Deserialize, ToSchema)]
pub (in crate::api) struct SchematicBuilder {
    schematic_name: String,
    game_version: i32,
    create_version: i32,
}

#[derive(Debug, Deserialize, ToSchema)]
pub (in crate::api) struct UpdateSchematic {
    schematic_name: Option<String>,
    game_version: Option<i32>,
    create_version: Option<i32>,
}

#[derive(Deserialize, ToSchema)]
pub (in crate::api) struct SearchQuery {
    limit: Option<i64>,
    offset: Option<i64>,
    term: String
}

pub (in crate::api::v1) fn configure() -> Router<ApiContext> {
    Router::new()
        .route("/schematics", 
            get(search_schematics)
            .post(upload_schematic)
        )
        .route(
            "/schematics/:id",
            get(get_schematic_by_id)
            .patch(update_schematic_by_id)
            .delete(delete_schematic_by_id)
        )
}

#[utoipa::path(
    get,
    path = "/schematics/{id}",
    context_path = "/api/v1",
    tag = "v1",
    params(
        ("id" = String, Path, description = "The id of the schematic to fetch")
    ),
    responses(
        (status = 200, description = "Successfully retrieved the schematic", body = Schematic, content_type = "application/json"),
        (status = 404, description = "A schematic with that id was not found"),
        (status = 500, description = "An internal server error occurred")
    ),
    security(())
)]
async fn get_schematic_by_id(
    State(ctx): State<ApiContext>,
    Path(id): Path<i64>
) -> ApiResult<Json<Schematic>> {
    sqlx::query_as!(
        Schematic,
        r#"
        select schematic_id, schematic_name, 
                game_version, create_version,
                downloads, author
        from schematics
        where schematic_id = $1
        "#,
        id
    )
    .fetch_optional(&ctx.pool)
    .await?
    .ok_or(ApiError::NotFound)
    .map(Json)
}

#[utoipa::path(
    patch,
    path = "/schematics/{id}",
    context_path = "/api/v1",
    tag = "v1",
    params(
        ("id" = String, Path, description = "The id of the schematic to update")
    ),
    request_body(
        content = UpdateSchematic, description = "The values to update", content_type = "application/json"
    ),
    responses(
        (status = 200, description = "Successfully retrieved the schematic", body = Schematic, content_type = "application/json"),
        (status = 401, description = "You need to be logged in to update a schematic"),
        (status = 403, description = "You do not have permission to update this schematic"),
        (status = 404, description = "A schematic with that id was not found"),
        (status = 422, description = "A schematic with the new name already exists"),
        (status = 500, description = "An internal server error occurred")
    ),
    security(("session_cookie" = []))
)]
async fn update_schematic_by_id(
    State(ctx): State<ApiContext>,
    Path(id): Path<i64>,
    Json(schematic): Json<UpdateSchematic>,
) -> ApiResult<Json<Schematic>> {
    let mut transaction = ctx.pool.begin().await?;

    let schematic = sqlx::query_as!(
        Schematic,
        r#"
        update schematics
            set
                schematic_name = coalesce($1, schematic_name),
                game_version = coalesce($2, game_version),
                create_version = coalesce($3, create_version)
            where schematic_id = $4
            returning
                schematic_id,
                schematic_name,
                game_version,
                create_version,
                author,
                downloads
        "#,
        schematic.schematic_name,
        schematic.game_version,
        schematic.create_version,
        id
    )
    .fetch_optional(&mut *transaction)
    .await?
    .ok_or(ApiError::NotFound)
    .map(Json)?;

    transaction.commit().await?;

    Ok(schematic)
}

#[utoipa::path(
    delete,
    path = "/schematics/{id}",
    context_path = "/api/v1",
    tag = "v1",
    params(
        ("id" = String, Path, description = "The id of the schematic to remove")
    ),
    responses(
        (status = 200, description = "Successfully deleted the schematic"),
        (status = 401, description = "You need to be logged in to delete a schematic"),
        (status = 403, description = "You do not have permission to delete this schematic"),
        (status = 404, description = "A schematic with that id was not found"),
        (status = 422, description = "A schematic with the new name already exists"),
        (status = 500, description = "An internal server error occurred")
    ),
    security(("session_cookie" = []))
)]
async fn delete_schematic_by_id(
    State(ctx): State<ApiContext>,
    Path(id): Path<i64>,
    session: Session
) -> ApiResult<()> {
    let result = sqlx::query!(
        r#"
        with deleted_schematic as (
            delete from schematics
            where schematic_id = $1 and author = $2
            returning 1
        )
        select
            exists(select 1 from schematics where schematic_id = $1) "existed",
            exists(select 1 from deleted_schematic) "deleted"
        "#,
        id,
        session.user_id
    )
    .fetch_one(&ctx.pool)
    .await?;

    if result.deleted.unwrap_or_default() {
        // The schematic was removed
        Ok(())
    } else if result.existed.unwrap_or_default() {
        // The schematic was not removed but did exist
        Err(ApiError::Forbidden)
    } else {
        // The schematic did not exist in the first place
        Err(ApiError::NotFound)
    }
}

#[utoipa::path(
    post,
    path = "/schematics",
    context_path = "/api/v1",
    tag = "v1",
    request_body(
        content = SchematicBuilder, description = "Information about the new schematic", content_type = "application/json"
    ),
    responses(
        (status = 200, description = "Successfully uploaded the schematic", body = Schematic, content_type = "application/json"),
        (status = 401, description = "You must be logged in to upload a schematic"),
        (status = 403, description = "You do not have permission to upload a schematic"),
        (status = 500, description = "An error occurred while uploading the schematic")
    ),
    security(("session_cookie" = []))
)]
async fn upload_schematic(
    State(ctx): State<ApiContext>,
    session: Session,
    Json(schematic): Json<SchematicBuilder>,
) -> ApiResult<Json<Schematic>> {
    let mut transaction = ctx.pool.begin().await?;

    let schematic = sqlx::query_as!(
        Schematic,
        r#"
        insert into schematics (
            schematic_name, author,
            game_version, create_version 
        )
        values (
            $1, $2, $3, $4
        )
        returning
            schematic_id,
            schematic_name,
            game_version,
            create_version,
            author,
            downloads
        "#,
        schematic.schematic_name,
        session.user_id,
        schematic.game_version,
        schematic.create_version
    )
    .fetch_one(&mut *transaction)
    .await
    .map(Json)?;

    transaction.commit().await?;

    Ok(schematic)
}

#[utoipa::path(
    get,
    path = "/schematics",
    context_path = "/api/v1",
    tag = "v1",
    params(
        ("id" = SearchQuery, Query, description = "The id of the schematic to fetch")
    ),
    responses(
        (status = 200, description = "Successfully retrieved the schematics", body = [Schematic], content_type = "application/json"),
        (status = 500, description = "An internal server error occurred")
    ),
    security(())
)]
async fn search_schematics(
    State(ctx): State<ApiContext>,
    Query(query): Query<SearchQuery>,
) -> ApiResult<Json<Vec<Schematic>>> {
    let schematics = sqlx::query_as!(
        Schematic,
        r#"
        select schematic_id, schematic_name, 
               game_version, create_version, 
               downloads, author
        from schematics
        where schematic_name like $1
        limit $2
        offset $3
        "#,
        query.term,
        query.limit.unwrap_or(20),
        query.offset.unwrap_or(0)
    )
    .fetch_all(&ctx.pool)
    .await
    .map(Json)?;

    Ok(schematics)
}