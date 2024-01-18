use poem::web::Data;
use poem_openapi::param::Path;
use poem_openapi::payload::Json;
use poem_openapi_derive::{Object, Multipart, OpenApi};
use uuid::Uuid;

use crate::authentication::schemes::Session;
use crate::middleware::files::FileUpload;
use crate::storage;
use crate::api::ApiContext;
use crate::response::ApiResult;
use crate::error::ApiError;

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
        let file_name = form.image.file_name.ok_or(ApiError::BadRequest)?;
        let mut transaction = ctx.pool.begin().await?;

        let schematic_meta = sqlx::query!(
            r#"select author from schematics where schematic_id = $1"#,
            schematic_id
        )
        .fetch_optional(&mut *transaction)
        .await?
        .ok_or(ApiError::NotFound)?;

        if schematic_meta.author != user_id {
            return Err(ApiError::Forbidden);
        }

        sqlx::query!(
            r#"
            update schematics
            set images = array_append(images, $1)
            where schematic_id = $2
            "#,
            file_name,
            schematic_id
        )
        .execute(&mut *transaction)
        .await?;

        let location = storage::schematic_image_path(&schematic_id);

        storage::upload::save_image(&location, &file_name, &form.image.contents)?;
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
    ) -> ApiResult<()> {
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

        sqlx::query_as!(
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
    
        let path = storage::schematic_image_path(&schematic_id);
    
        tokio::fs::remove_file(path.join(form.file_name))
            .await
            .map_err(anyhow::Error::new)?;
    
        transaction.commit().await?;
    
        Ok(())
    }
}