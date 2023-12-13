use std::path::PathBuf;

use axum::routing::get;
use axum::{Router, Json};
use axum::extract::{Path, State};
use axum::body::Bytes;
use axum_typed_multipart::{FieldData, TypedMultipart, TryFromMultipart};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::error::ApiError;
use crate::models::schematic::Schematic;
use crate::models::user::{User, Permissions};
use crate::storage::{UPLOAD_PATH, SCHEMATIC_PATH};
use crate::response::ApiResult;
use crate::api::ApiContext;

pub (in crate::api::v1) fn configure() -> Router<ApiContext> {
    Router::new()
        .route(
            "/schematics/:id/files", 
            get(get_files_from_schematic)
            .post(upload_file_to_schematic)
            .delete(remove_file_from_schematic)
        )
}

#[derive(Serialize, Debug, ToSchema)]
pub (in crate::api) struct Files {
    #[schema(min_items=1)]
    pub files: Vec<String>
}

#[derive(TryFromMultipart, Debug, ToSchema)]
pub (in crate::api) struct UploadFile {
    #[form_data(limit = "2KiB")]
    #[schema(value_type=String, format=Binary)]
    pub file: FieldData<Bytes>
}

#[derive(TryFromMultipart, Debug, ToSchema)]
pub (in crate::api) struct DeleteFile {
    #[schema(example="my_schematic.nbt")]
    pub file_name: String
}

#[utoipa::path(
    get,
    path = "/schematics/{schematic_id}/files",
    context_path = "/api/v1",
    tag = "v1",
    params(
        ("schematic_id" = Uuid, Path, description = "The id of the schematic to fetch files from"),
    ),
    responses(
        (status = 200, description = "Successfully retrieved the images", body = Files, content_type = "application/json"),
        (status = 500, description = "An internal server error occurred")
    ),
    security(())
)]
async fn get_files_from_schematic(
    Path(schematic_id): Path<Uuid>,
    State(ctx): State<ApiContext>
) -> ApiResult<Json<Files>> {
    sqlx::query_as!(
        Files,
        r#"
        select files
        from schematics
        where schematic_id = $1
        "#,
        schematic_id
    )
    .fetch_optional(&ctx.pool)
    .await?
    .ok_or(ApiError::NotFound)
    .map(Json)
}

#[utoipa::path(
    post,
    path = "/schematics/{schematic_id}/files",
    context_path = "/api/v1",
    tag = "v1",
    params(
        ("schematic_id" = Uuid, Path, description = "The id of the schematic to upload a file to")
    ),
    request_body(
        content = UploadFile, description = "The new schematic file", content_type = "multipart/form-data"
    ),
    responses(
        (status = 200, description = "Successfully uploaded the file to the schematic"),
        (status = 401, description = "You need to be logged in to upload a new file to a schematic"),
        (status = 403, description = "You do not have permission to add a file to this schematic"),
        (status = 404, description = "A schematic with that id was not found"),
        (status = 409, description = "This schematic already has a file with that name"),
        (status = 500, description = "An internal server error occurred")
    ),
    security(("session_cookie" = []))
)]
async fn upload_file_to_schematic(
    Path(schematic_id): Path<Uuid>,
    State(ctx): State<ApiContext>,
    user: User,
    TypedMultipart(form): TypedMultipart<UploadFile>
) -> ApiResult<()> {
    let file_name = form.file.metadata.file_name.ok_or(ApiError::BadRequest)?;
    let mut transaction = ctx.pool.begin().await?;
    
    Schematic::check_user_permissions(user, &schematic_id, Permissions::MODERATE_POSTS, &mut *transaction).await?;
    let sanitized = sanitize_filename::sanitize(file_name);

    sqlx::query!(
        r#"
        update schematics
            set 
                files = array_append(files, $1)
            where 
                schematic_id = $2
        "#,
        sanitized,
        schematic_id
    )
    .execute(&mut *transaction)
    .await?;

    let mut path = PathBuf::from(UPLOAD_PATH);
    path.push(schematic_id.to_string());
    path.push(SCHEMATIC_PATH);

    let file = path.join(sanitized);
    
    if file.exists() {
        return Err(ApiError::Conflict);
    }

    tokio::fs::write(file, form.file.contents)
        .await
        .map_err(anyhow::Error::new)?;
    
    transaction.commit().await?;

    Ok(())
}

#[utoipa::path(
    delete,
    path = "/schematics/{schematic_id}/files",
    context_path = "/api/v1",
    tag = "v1",
    params(
        ("schematic_id" = Uuid, Path, description = "The id of the schematic to remove a file from")
    ),
    request_body(
        content = DeleteImage, description = "The name of the file to remove", content_type = "multipart/form-data"
    ),
    responses(
        (status = 200, description = "Successfully deleted the file from the schematic", body = Images, content_type = "application/json"),
        (status = 401, description = "You need to be logged in to remove a file from a schematic"),
        (status = 403, description = "You do not have permission to remove a file from this schematic"),
        (status = 404, description = "A schematic with that id or a file with that name was not found"),
        (status = 500, description = "An internal server error occurred")
    ),
    security(("session_cookie" = []))
)]
async fn remove_file_from_schematic(
    Path(schematic_id): Path<Uuid>,
    State(ctx): State<ApiContext>,
    user: User,
    TypedMultipart(form): TypedMultipart<DeleteFile>
) -> ApiResult<Json<Files>> {
    let mut transaction = ctx.pool.begin().await?;
    
    Schematic::check_user_permissions(user, &schematic_id, Permissions::MODERATE_POSTS, &mut *transaction).await?;
    let file_name = sanitize_filename::sanitize(form.file_name);

    let files = sqlx::query_as!(
        Files,
        r#"
        update schematics
            set 
                files = array_remove(files, $1)
            where 
                schematic_id = $2
                and array_length(files, 1) > 2
        returning files
        "#,
        file_name,
        schematic_id
    )
    .fetch_optional(&mut *transaction)
    .await?
    .ok_or(ApiError::NotFound)?;

    let mut path = PathBuf::from(UPLOAD_PATH);
    path.push(schematic_id.to_string());
    path.push(SCHEMATIC_PATH);

    tokio::fs::remove_file(path.join(file_name))
        .await
        .map_err(anyhow::Error::new)?;

    transaction.commit().await?;

    Ok(Json(files))
}