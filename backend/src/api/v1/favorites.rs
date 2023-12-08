use axum::Router;
use axum::extract::{State, Path};
use axum::routing::{post, get};
use axum::Json;
use uuid::Uuid;

use crate::api::ApiContext; 
use crate::authentication::session::Session;
use crate::response::ApiResult;
use crate::models::schematic::Schematic;

pub (in crate::api::v1) fn configure() -> Router<ApiContext> {
    Router::new()
        .route(
            "/schematics/favorites", 
            get(get_favorites)
        )
        .route(
            "/schematics/:id/favorite", 
        post(favorite_schematic)
            .delete(unfavorite_schematic)
        )
}

#[utoipa::path(
    get,
    path = "/schematics/favorites",
    context_path = "/api/v1",
    tag = "v1",
    responses(
        (status = 200, description = "Successfully retrieved the schematics", body = [Schematic], content_type = "application/json"),
        (status = 401, description = "You need to be logged in to view your favourite schematics"),
        (status = 500, description = "An internal server error occurred")
    ),
    security(("session_cookie" = []))
)]
async fn get_favorites(
    State(ctx): State<ApiContext>,
    session: Session,
) -> ApiResult<Json<Vec<Schematic>>> {
    let schematics = sqlx::query_as!(
        Schematic,
        r#"
        select schematic_id, schematic_name,
               body, files, images, author,
               create_version_id, downloads,
               game_version_id
        from favorites
        inner join schematics
        using (schematic_id)
        where user_id = $1
        "#,
        session.user_id
    )
    .fetch_all(&ctx.pool)
    .await?;

    Ok(Json(schematics))
}

#[utoipa::path(
    post,
    path = "/schematics/{schematic_id}/favorite",
    context_path = "/api/v1",
    tag = "v1",
    params(
        ("schematic_id" = Uuid, Path, description = "The id of the schematic to favorite")
    ),
    responses(
        (status = 200, description = "Successfully favorited the schematic"),
        (status = 401, description = "You need to be logged in to favorite a schematic"),
        (status = 500, description = "An internal server error occurred")
    ),
    security(("session_cookie" = []))
)]
async fn favorite_schematic(
    State(ctx): State<ApiContext>,
    session: Session,
    Path(schematic_id): Path<Uuid>,
) -> ApiResult<()> {
    sqlx::query!(
        r#"
        insert into favorites (
            schematic_id, user_id
        )
        values (
            $1, $2
        )
        on conflict do nothing
        "#,
        schematic_id,
        session.user_id
    )
    .execute(&ctx.pool)
    .await?;

    Ok(())
}

#[utoipa::path(
    delete,
    path = "/schematics/{schematic_id}/favorite",
    context_path = "/api/v1",
    tag = "v1",
    params(
        ("schematic_id" = Uuid, Path, description = "The id of the schematic to unfavorite")
    ),
    responses(
        (status = 200, description = "Successfully unfavorited the schematic"),
        (status = 401, description = "You need to be logged in to unfavorite a schematic"),
        (status = 500, description = "An internal server error occurred")
    ),
    security(("session_cookie" = []))
)]
async fn unfavorite_schematic(
    State(ctx): State<ApiContext>,
    session: Session,
    Path(schematic_id): Path<Uuid>,
) -> ApiResult<()> {
    sqlx::query!(
        r#"
        delete from favorites
        where schematic_id = $1
        and user_id = $2
        "#,
        schematic_id,
        session.user_id
    )
    .execute(&ctx.pool)
    .await?;

    Ok(())
}
