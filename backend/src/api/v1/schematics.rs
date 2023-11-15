use axum::routing::get;
use axum::{Router, Json};
use axum::extract::{State, Path};
use utoipa::ToSchema;

use crate::error::ApiError;
use crate::response::ApiResult;
use crate::cli::server::ApiContext;

#[derive(Debug, Serialize)]
pub struct Schematic {
    schematic_id: i64,
    schematic_name: String,
    game_version: i32,
    create_version: i32,
    downloads: i64,
}

#[derive(Debug, Serialize)]
pub struct SchematicFromQuery {
    schematic_id: i64,
    schematic_name: String,
    game_version: i32,
    create_version: i32,
    downloads: i64,
}

#[derive(Debug, Deserialize)]
pub struct SchematicBuilder {
    schematic_name: String,
    game_version: i32,
    create_version: i32,
    required_mods: Vec<String>,
    tags: Vec<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateSchematic {
    schematic_name: Option<String>,
    game_version: Option<i32>,
    create_version: Option<i32>,
    required_mods: Option<Vec<String>>,
    tags: Option<Vec<String>>,
}

pub async fn configure() -> Router<ApiContext> {
    Router::new()
        .route(
            "/schematics/:id",
            get(get_schematic_by_id)
            .patch(update_schematic_by_id)
            .delete(delete_schematic_by_id)
        )
}


async fn get_schematic_by_id(
    State(ctx): State<ApiContext>,
    Path(id): Path<i64>
) -> ApiResult<Json<Schematic>> {
    sqlx::query_as!(
        Schematic,
        r#"
        select schematic_id, schematic_name, 
                game_version, create_version, 
                downloads
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
                downloads
        "#,
        schematic.schematic_name,
        schematic.game_version,
        schematic.game_version,
        id
    )
    .fetch_optional(&mut *transaction)
    .await?
    .ok_or(ApiError::NotFound)
    .map(Json)?;

    transaction.commit().await?;

    Ok(schematic)
}

async fn delete_schematic_by_id(
    State(ctx): State<ApiContext>,
    Path(id): Path<i64>,
) -> ApiResult<()> {
    let result = sqlx::query!(
        r#"
        with deleted_schematic as (
            delete from schematics
            where schematic_id = $1
            returning 1
        )
        select
            exists(select 1 from schematics where schematic_id = $1) "existed",
            exists(select 1 from deleted_schematic) "deleted"
        "#,
        id
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