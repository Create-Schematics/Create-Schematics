use std::path::PathBuf;

use poem::web::Data;
use poem_openapi::param::Path;
use poem_openapi::payload::Json;
use poem_openapi_derive::{Object, Multipart, OpenApi};
use uuid::Uuid;

use crate::middleware::files::FileUpload;
use crate::storage::{UPLOAD_PATH, IMAGE_PATH};
use crate::api::ApiContext;
use crate::response::ApiResult;
use crate::error::ApiError;
use crate::models::schematic::Schematic;
use crate::models::user::{Permissions, User};

const MAX_IMAGE_SIZE: u64 = 1024 * 1024 * 2; // 2mb

pub (in crate::api::v1) struct ImageApi;

#[derive(Serialize, Debug, Object)]
pub (in crate::api) struct Images {
    pub images: Vec<String>
}

#[derive(Multipart, Debug)]
pub (in crate::api) struct UploadImage {
    pub image: FileUpload
}

#[derive(Multipart, Debug)]
pub (in crate::api) struct DeleteImage {
    pub file_name: String
}

#[OpenApi(prefix_path="/api/v1")]
impl ImageApi {

    /// Fetches the file names of all images associated with a given schematic
    /// 
    /// Note this does not return the image files themselves they can be
    /// retrieved from the static file endpoint here
    /// `GET /upload/schematics/{schematic_id}/images/{image_name}.{extension}`
    /// 
    #[oai(path="/schematics/:id/images", method="get")]
    async fn get_images_from_schematic(
        &self,
        Data(ctx): Data<&ApiContext>,    
        Path(schematic_id): Path<Uuid>
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

    /// Uploads a new image to an existing schematic, for supported image formats
    /// see the image crate as this is used to ensure that images are valid.
    /// 
    /// https://github.com/image-rs/image?tab=readme-ov-file#supported-image-formats
    /// 
    /// File names cannot overlap, if an image with a given name is already added
    /// to the schematic then the request will be rejected with a `409 Conflict`
    /// response.
    /// 
    /// Aswell as this file names cannot contain profanity if the file name is deemed
    /// to be profane the request will be rejected 
    /// 
    #[oai(path="/schematics/:id/images", method="post")]
    async fn upload_image_to_schematic(
        &self,
        Data(ctx): Data<&ApiContext>,  
        Path(schematic_id): Path<Uuid>,
        user: User,
        form: UploadImage
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
        
        if file.exists() || form.image.contents.len() > MAX_IMAGE_SIZE {
            return Err(ApiError::Conflict);
        }
    
        image::load_from_memory(&form.image.contents)
            .map_err(|_| ApiError::BadRequest)?
            .save(path)
            .map_err(anyhow::Error::new)?;
    
        transaction.commit().await?;
    
        Ok(())
    }

    /// Removes an image from a schematic
    /// 
    /// Each schematic must have at least one image so requests to remove the
    /// final one will be rejected with a `400 Bad Request` response.
    /// 
    /// This endpoint requires the user to either own the schematic or have
    /// permissions to moderate schematics.
    /// 
    #[oai(path="/schematics/:id/images", method="delete")]
    async fn remove_image_from_schematic(
        &self,
        Data(ctx): Data<&ApiContext>,   
        Path(schematic_id): Path<Uuid>,
        user: User,
        form: DeleteImage
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
}