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
    pub images: Vec<String>
}

#[derive(TryFromMultipart, Debug, ToSchema)]
pub (in crate::api) struct UploadImage {
    pub image: FieldData<Bytes>
}

#[derive(TryFromMultipart, Debug, ToSchema)]
pub (in crate::api) struct DeleteImage {
    pub file_name: String
}

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