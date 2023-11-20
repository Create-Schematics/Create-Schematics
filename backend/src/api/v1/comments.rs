use axum::extract::{State, Path, Query};
use axum::Json;
use axum::Router;
use axum::routing::get;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::api::ApiContext;
use crate::response::ApiResult;
use crate::models::comment::Comment;
use crate::error::ApiError;
use crate::authentication::session::Session;

pub (in crate::api::v1) fn configure() -> Router<ApiContext> {
    Router::new()
        .route(
            "/schematics/:id/comments", 
            get(get_comments_by_schematic)
            .post(post_comment)
        )
        .route(
            "/comments/:id",
            get(get_comment_by_id)
            .patch(update_comment_by_id)
            .delete(delete_comment_by_id)    
        )
}

#[derive(Deserialize, ToSchema)]
pub (in crate::api) struct PaginationQuery {
    limit: Option<i64>,
    offset: Option<i64>,
}

#[derive(Serialize, ToSchema)]
pub (in crate::api) struct FullComment {
    comment_id: i64,
    comment_author: Uuid,
    comment_body: String,
    schematic_id: i64,
    author_username: String
}

#[derive(Deserialize, ToSchema)]
pub (in crate::api) struct CommentBuilder {
    comment_body: String
}

#[derive(Deserialize, ToSchema)]
pub (in crate::api) struct UpdateComment {
    comment_body: Option<String>
}

#[utoipa::path(
    get,
    path = "/schematics/{id}/comments",
    context_path = "/api/v1",
    tag = "v1",
    params(
        ("id" = SearchQuery, Query, description = "The id of the schematic to fetch the comments from")
    ),
    responses(
        (status = 200, description = "Successfully retrieved the comments", body = [FullComment], content_type = "application/json"),
        (status = 500, description = "An internal server error occurred")
    ),
    security(())
)]
async fn get_comments_by_schematic(
    State(ctx): State<ApiContext>,
    Query(query): Query<PaginationQuery>,
    Path(schematic_id): Path<i64>,
) -> ApiResult<Json<Vec<FullComment>>> {
    let schematics = sqlx::query_as!(
        FullComment,
        r#"
        select comment_id, comment_author,
               comment_body, schematic_id,
               username as author_username
        from comments
        inner join users
        on comment_author = user_id
        where schematic_id = $1
        limit $2 offset $3
        "#,
        schematic_id,
        query.limit.unwrap_or(20),
        query.offset.unwrap_or(0)
    )
    .fetch_all(&ctx.pool)
    .await?;

    Ok(Json(schematics))
}

#[utoipa::path(
    get,
    path = "/comments/{id}",
    context_path = "/api/v1",
    tag = "v1",
    params(
        ("id" = String, Path, description = "The id of the comment to fetch")
    ),
    responses(
        (status = 200, description = "Successfully retrieved the comment", body = FullComment, content_type = "application/json"),
        (status = 404, description = "A comment with that id was not found"),
        (status = 500, description = "An internal server error occurred")
    ),
    security(())
)]
async fn get_comment_by_id(
    State(ctx): State<ApiContext>,
    Path(comment_id): Path<i64>,
) -> ApiResult<Json<FullComment>> {
    sqlx::query_as!(
        FullComment,
        r#"
        select comment_id, comment_author,
               comment_body, schematic_id,
               username as author_username
        from comments
        inner join users
        on comment_author = user_id
        where comment_id = $1
        "#,
        comment_id,
    )
    .fetch_optional(&ctx.pool)
    .await?
    .ok_or(ApiError::NotFound)
    .map(Json)
}

#[utoipa::path(
    post,
    path = "/schematics/{id}/comments",
    context_path = "/api/v1",
    tag = "v1",
    request_body(
        content = CommentBuilder, description = "The text of the comment", content_type = "application/json"
    ),
    responses(
        (status = 200, description = "Successfully added comment to schematic", body = Comment, content_type = "application/json"),
        (status = 401, description = "You must be logged in to comment"),
        (status = 403, description = "You do not have permission to comment"),
        (status = 500, description = "An error occurred while uploading the comment")
    ),
    security(("session_cookie" = []))
)]
async fn post_comment(
    State(ctx): State<ApiContext>,
    Path(schematic_id): Path<i64>,
    session: Session,
    Json(builder): Json<CommentBuilder>
) -> ApiResult<Json<Comment>> {
    let mut transaction = ctx.pool.begin().await?;

    let schematic = sqlx::query_as!(
        Comment,
        r#"
        insert into comments (
            comment_author, comment_body,
            schematic_id
        )
        values (
            $1, $2, $3
        )
        returning
            comment_id,
            comment_author,
            comment_body,
            schematic_id
        "#,
        session.user_id,
        builder.comment_body,
        schematic_id
    )
    .fetch_one(&mut *transaction)
    .await?;

    transaction.commit().await?;

    Ok(Json(schematic))
}

#[utoipa::path(
    patch,
    path = "/comments/{id}",
    context_path = "/api/v1",
    tag = "v1",
    params(
        ("id" = String, Path, description = "The id of the comment to update")
    ),
    request_body(
        content = UpdateComment, description = "The new body of the comment", content_type = "application/json"
    ),
    responses(
        (status = 200, description = "Successfully updated the comment", body = Schematic, content_type = "application/json"),
        (status = 401, description = "You need to be logged in to update a comment"),
        (status = 403, description = "You do not have permission to update this comment"),
        (status = 404, description = "A comment with that id was not found"),
        (status = 500, description = "An internal server error occurred")
    ),
    security(("session_cookie" = []))
)]
async fn update_comment_by_id(
    State(ctx): State<ApiContext>,
    Path(comment_id): Path<i64>,
    session: Session,
    Json(update): Json<UpdateComment>
) -> ApiResult<Json<Comment>> {
    let mut transaction = ctx.pool.begin().await?;

    let comment = sqlx::query_as!(
        Comment,
        r#"
        update comments
        set comment_body = coalesce($1, comment_body)
        where comment_author = $2
        and comment_id = $3
        returning
            comment_id,
            comment_author,
            comment_body,
            schematic_id
        "#,
        update.comment_body,
        session.user_id,
        comment_id
    )
    .fetch_optional(&mut *transaction)
    .await?
    .ok_or(ApiError::NotFound)?;

    transaction.commit().await?;

    Ok(Json(comment))
}

#[utoipa::path(
    delete,
    path = "/comments/{id}",
    context_path = "/api/v1",
    tag = "v1",
    params(
        ("id" = String, Path, description = "The id of the comment to remove")
    ),
    responses(
        (status = 200, description = "Successfully deleted the comment"),
        (status = 401, description = "You need to be logged in to delete a comment"),
        (status = 403, description = "You do not have permission to delete this comment"),
        (status = 404, description = "A comment with that id was not found"),
        (status = 500, description = "An internal server error occurred")
    ),
    security(("session_cookie" = []))
)]
async fn delete_comment_by_id(
    State(ctx): State<ApiContext>,
    Path(comment_id): Path<i64>,
    session: Session
) -> ApiResult<()> {
    let mut transaction = ctx.pool.begin().await?;

    let result = sqlx::query!(
        r#"
        with deleted_comment as (
            delete from comments
            where comment_author = $1
            and comment_id = $2
            returning 1
        )
        select
            exists(select 1 from comments where comment_id = $2) "existed",
            exists(select 1 from deleted_comment) "deleted"
        "#,
        session.user_id,
        comment_id
    )
    .fetch_one(&mut *transaction)
    .await?;

    transaction.commit().await?;

    if result.deleted.unwrap_or_default() {
        // The comment existed, was owned by the user and was succesfully removed
        Ok(())
    } else if result.existed.unwrap_or_default() {
        // The comment existed, but was not owned by the user
        Err(ApiError::Forbidden)
    } else {
        // The comment did not exist in the first place
        Err(ApiError::NotFound)
    }
}    