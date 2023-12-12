use std::path::PathBuf;

use axum::{Router, Json};
use axum::body::Bytes;
use axum::extract::{State, Path};
use axum::routing::get;
use axum_typed_multipart::{TryFromMultipart, FieldData, TypedMultipart};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::storage::{UPLOAD_PATH, IMAGE_PATH};
use crate::api::ApiContext;
use crate::response::ApiResult;
use crate::error::ApiError;
use crate::models::schematic::Schematic;
use crate::models::user::{Permissions, User};

pub (in crate::api::v1) fn configure() -> Router<ApiContext> {
    Router::new()
        .route(
            "/schematics/:id/images",
            get(get_images_from_schematic)
            .post(upload_image_to_schematic)
            .delete(remove_image_from_schematic)
        )
}

#[derive(Serialize, Debug, ToSchema)]
pub (in crate::api) struct Images {
    #[schema(min_length=1)]
    pub images: Vec<String>
}

#[derive(TryFromMultipart, Debug, ToSchema)]
pub (in crate::api) struct UploadImage {
    #[form_data(limit = "2MiB")]
    #[schema(value_type=String, format=Binary)]
    pub image: FieldData<Bytes>
}

#[derive(TryFromMultipart, Debug, ToSchema)]
pub (in crate::api) struct DeleteImage {
    #[schema(example="my_image.webp")]
    pub file_name: String
}

#[utoipa::path(
    get,
    path = "/schematics/{schematic_id}/images",
    context_path = "/api/v1",
    tag = "v1",
    params(
        ("schematic_id" = Uuid, Path, description = "The id of the schematic to fetch images from"),
    ),
    responses(
        (status = 200, description = "Successfully retrieved the images", body = Images, content_type = "application/json"),
        (status = 500, description = "An internal server error occurred")
    ),
    security(())
)]
async fn get_images_from_schematic(
    Path(schematic_id): Path<Uuid>,
    State(ctx): State<ApiContext>
) -> ApiResult<Json<Images>> {
    sqlx::query_as!(
        Images,
        r#"
        select images
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
    path = "/schematics/{schematic_id}/images",
    context_path = "/api/v1",
    tag = "v1",
    params(
        ("schematic_id" = Uuid, Path, description = "The id of the schematic to upload an image to")
    ),
    request_body(
        content = UploadImage, description = "The new image", content_type = "multipart/form-data"
    ),
    responses(
        (status = 200, description = "Successfully uploaded image to schematic"),
        (status = 401, description = "You need to be logged in to upload an image to a schematic"),
        (status = 403, description = "You do not have permission to add an image to this schematic"),
        (status = 404, description = "A schematic with that id was not found"),
        (status = 409, description = "This schematic already has an image with that name"),
        (status = 500, description = "An internal server error occurred")
    ),
    security(("session_cookie" = []))
)]
async fn upload_image_to_schematic(
    Path(schematic_id): Path<Uuid>,
    State(ctx): State<ApiContext>,
    user: User,
    TypedMultipart(form): TypedMultipart<UploadImage>
) -> ApiResult<()> {
    let file_name = form.image.metadata.file_name.ok_or(ApiError::BadRequest)?;
    let mut transaction = ctx.pool.begin().await?;
    
    Schematic::check_user_permissions(user, &schematic_id, Permissions::MODERATE_POSTS, &mut *transaction).await?;
    let sanitized = sanitize_filename::sanitize(file_name);

    sqlx::query!(
        r#"
        update schematics
            set 
                images = array_append(images, $1)
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
    path.push(IMAGE_PATH);

    let file = path.join(sanitized);
    
    if file.exists() {
        return Err(ApiError::Conflict);
    }

    image::load_from_memory(&form.image.contents)
        .map_err(|_| ApiError::BadRequest)?
        .save(path)
        .map_err(anyhow::Error::new)?;

    transaction.commit().await?;

    Ok(())
}

#[utoipa::path(
    delete,
    path = "/schematics/{schematic_id}/images",
    context_path = "/api/v1",
    tag = "v1",
    params(
        ("schematic_id" = Uuid, Path, description = "The id of the schematic to remove an image from")
    ),
    request_body(
        content = DeleteImage, description = "The name of the image to remove", content_type = "multipart/form-data"
    ),
    responses(
        (status = 200, description = "Successfully deleted image from the schematic", body = Images, content_type = "application/json"),
        (status = 401, description = "You need to be logged in to remove an image from a schematic"),
        (status = 403, description = "You do not have permission to remove an image from this schematic"),
        (status = 404, description = "A schematic with that id or an image with that name was not found"),
        (status = 500, description = "An internal server error occurred")
    ),
    security(("session_cookie" = []))
)]
async fn remove_image_from_schematic(
    Path(schematic_id): Path<Uuid>,
    State(ctx): State<ApiContext>,
    user: User,
    TypedMultipart(form): TypedMultipart<DeleteImage>
) -> ApiResult<Json<Images>> {
    let mut transaction = ctx.pool.begin().await?;
    
    Schematic::check_user_permissions(user, &schematic_id, Permissions::MODERATE_POSTS, &mut *transaction).await?;
    let file_name = sanitize_filename::sanitize(form.file_name);

    let images = sqlx::query_as!(
        Images,
        r#"
        update schematics
            set 
                images = array_remove(images, $1)
            where 
                schematic_id = $2
                and array_length(images, 1) > 2
        returning images
        "#,
        file_name,
        schematic_id
    )
    .fetch_optional(&mut *transaction)
    .await?
    .ok_or(ApiError::NotFound)?;

    let mut path = PathBuf::from(UPLOAD_PATH);
    path.push(schematic_id.to_string());
    path.push(IMAGE_PATH);

    tokio::fs::remove_file(path.join(file_name))
        .await
        .map_err(anyhow::Error::new)?;

    transaction.commit().await?;

    Ok(Json(images))
}