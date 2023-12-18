use std::path::PathBuf;

use poem::web::Data;
use poem_openapi::param::Path;
use poem_openapi::payload::Json;
use poem_openapi_derive::{Object, Multipart, OpenApi};
use uuid::Uuid;

use crate::authentication::session::Session;
use crate::error::ApiError;
use crate::middleware::files::FileUpload;
use crate::storage::{UPLOAD_PATH, SCHEMATIC_PATH};
use crate::response::ApiResult;
use crate::api::ApiContext;

const MAX_FILE_SIZE: usize = 256 * 1024; // 256kb

pub struct FileApi;

#[derive(Serialize, Debug, Object)]
pub struct Files {
    #[oai(validator(min_items=1))]
    pub files: Vec<String>
}

#[derive(Multipart, Debug)]
pub struct UploadFile {
    pub file: FileUpload
}

#[derive(Multipart, Debug)]
pub struct DeleteFile {

    pub file_name: String
}

#[OpenApi(prefix_path="/v1")]
impl FileApi {

    /// Fetches the name of all uploaded schematic files on a given schematic
    /// 
    /// Note this does not return the schematic files themselves, they can be 
    /// retrieved from the static file endpoint like so filling in the schematic
    /// id for the given schematic and file_name for one of the values returned
    /// here `GET /upload/schematics/{schematic_id}/files/{file_name}.nbt`
    /// 
    #[oai(path = "/schematics/:schematic_id/files", method = "get")]
    async fn get_files_from_schematic(
        &self,
        Data(ctx): Data<&ApiContext>,    
        Path(schematic_id): Path<Uuid>,
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

    /// Uploads a new schematic file to a schematic, use this for schematics
    /// with multiple variations or parts not for many entirely different 
    /// schematics. 
    /// 
    /// This requires for the current user to be the owner of the given schematic
    /// and for this file name (after sanitization) to not be used already. If
    /// there are conflicting file names `422 Unprocessable Entity` will be returned
    /// with a message explaining this
    ///  
    #[oai(path = "/schematics/:schematic_id/files", method = "post")]
    async fn upload_file_to_schematic(
        &self,
        Data(ctx): Data<&ApiContext>,    
        Path(schematic_id): Path<Uuid>,
        Session(user_id): Session,
        form: UploadFile
    ) -> ApiResult<()> {
        let file_name = form.file.file_name.ok_or(ApiError::BadRequest)?;
        let sanitized = sanitize_filename::sanitize(file_name);
        
        if form.file.contents.len() > MAX_FILE_SIZE || !sanitized.ends_with(".nbt") {
            return Err(ApiError::BadRequest);
        }

        let mut path = PathBuf::from(UPLOAD_PATH);
        path.push(schematic_id.to_string());
        path.push(SCHEMATIC_PATH);
    
        let file = path.join(&sanitized);
        
        if file.exists() {
            return Err(ApiError::unprocessable_entity([("file", "a file with this name already exists")]));
        }

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
                    files = array_append(files, $1)
                where 
                    schematic_id = $2
            "#,
            sanitized,
            schematic_id
        )
        .execute(&mut *transaction)
        .await?;
        
        tokio::fs::write(file, form.file.contents)
            .await
            .map_err(anyhow::Error::new)?;

        transaction.commit().await?;
    
        Ok(())
    }

    /// Removes a schematic file from a schematic, at least one file must be
    /// present at all times. Requests to remove the last file will result in
    /// a `400 Bad Request` error
    /// 
    /// This requires the current to user to either own the schematic or have
    /// permissions to moderate schematics
    /// 
    #[oai(path = "/schematics/:schematic_id/files", method = "delete")]
    async fn remove_file_from_schematic(
        &self,
        Data(ctx): Data<&ApiContext>,   
        Path(schematic_id): Path<Uuid>,
        session: Session,
        form: DeleteFile
    ) -> ApiResult<Json<Files>> {
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
}