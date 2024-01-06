use std::path::PathBuf;

use poem::web::Data;
use poem_openapi::param::Path;
use poem_openapi::payload::Json;
use poem_openapi_derive::{Object, Multipart, OpenApi};
use uuid::Uuid;

use crate::authentication::session::Session;
use crate::middleware::files::FileUpload;
use crate::storage::{UPLOAD_PATH, IMAGE_PATH};
use crate::api::ApiContext;
use crate::response::ApiResult;
use crate::error::ApiError;

const MAX_IMAGE_SIZE: usize = 1024 * 1024 * 2; // 2mb

pub struct ImageApi;

#[derive(Serialize, Debug, Object)]
pub struct Images {
    pub images: Vec<String>
}

#[derive(Multipart, Debug)]
pub struct UploadImage {
    pub image: FileUpload
}

#[derive(Multipart, Debug)]
pub struct DeleteImage {
    pub file_name: String
}

#[OpenApi(prefix_path="/v1")]
impl ImageApi {

    /// Fetches the file names of all images associated with a given schematic
    /// 
    /// Note this does not return the image files themselves they can be
    /// retrieved from the static file endpoint here
    /// `GET /upload/schematics/{schematic_id}/images/{image_name}.{extension}`
    /// 
    #[oai(path="/schematics/:schematic_id/images", method="get")]
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
    #[oai(path="/schematics/:schematic_id/images", method="post")]
    async fn upload_image_to_schematic(
        &self,
        Data(ctx): Data<&ApiContext>,  
        Path(schematic_id): Path<Uuid>,
        Session(user_id): Session,
        form: UploadImage
    ) -> ApiResult<()> {
        let mut transaction = ctx.pool.begin().await?;
        
        let schematic_meta = sqlx::query!(
            r#"select author from schematics where schematic_id = $1"#,
            schematic_id
        )
        .fetch_optional(&mut *transaction)
        .await?
        .ok_or(ApiError::NotFound)?;
    
        if schematic_meta.author != user_id {
            return Err(ApiError::Unauthorized);
        }
        
        sqlx::query!(
            r#"
            update schematics
                set 
                    images = array_append(images, $1)
                where 
                    schematic_id = $2
            "#,
            form.image.file_name,
            schematic_id
        )
        .execute(&mut *transaction)
        .await?;
    
        let mut path = PathBuf::from(UPLOAD_PATH);
        path.push(schematic_id.to_string());
        path.push(IMAGE_PATH);
    
        let file = path.join(form.image.file_name);
        
        if file.exists() {
            return Err(ApiError::unprocessable_entity([("image", "a file with this name already exists")]));
        }

        if form.image.contents.len() > MAX_IMAGE_SIZE {
            return Err(ApiError::unprocessable_entity([("image", "file size too large")]));
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
    #[oai(path="/schematics/:schematic_id/images", method="delete")]
    async fn remove_image_from_schematic(
        &self,
        Data(ctx): Data<&ApiContext>,   
        Path(schematic_id): Path<Uuid>,
        session: Session,
        form: DeleteImage
    ) -> ApiResult<Json<Images>> {
        let mut transaction = ctx.pool.begin().await?;
        
        let schematic_meta = sqlx::query!(
            r#"select author from schematics where schematic_id = $1"#,
            schematic_id
        )
        .fetch_optional(&mut *transaction)
        .await?
        .ok_or(ApiError::NotFound)?;

        if schematic_meta.author != session.user_id() &&
                !session.is_moderator(&mut *transaction).await? {
            return Err(ApiError::Unauthorized);
        }

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
            form.file_name,
            schematic_id
        )
        .fetch_optional(&mut *transaction)
        .await?
        .ok_or(ApiError::NotFound)?;
    
        let mut path = PathBuf::from(UPLOAD_PATH);
        path.push(schematic_id.to_string());
        path.push(IMAGE_PATH);
    
        tokio::fs::remove_file(path.join(form.file_name))
            .await
            .map_err(anyhow::Error::new)?;
    
        transaction.commit().await?;
    
        Ok(Json(images))
    }
}