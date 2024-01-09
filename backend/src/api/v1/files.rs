use poem::web::Data;
use poem_openapi::param::Path;
use poem_openapi::payload::Json;
use poem_openapi_derive::{Object, Multipart, OpenApi};
use uuid::Uuid;

use crate::authentication::session::Session;
use crate::error::{ApiError, ResultExt};
use crate::middleware::files::FileUpload;
use crate::storage;
use crate::response::ApiResult;
use crate::api::ApiContext;

pub (in crate::api::v1) struct FileApi;

#[derive(Serialize, Debug, Object)]
pub (in crate::api::v1) struct File {
    pub file_id: Uuid,
    pub file_name: String,
    pub schematic_id: Uuid,
    pub requirements: Vec<String>
}

#[derive(Multipart, Debug)]
pub struct UploadFile {
    pub file: FileUpload
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
    ) -> ApiResult<Json<Vec<File>>> {
        let files = sqlx::query_as!(
            File,
            r#"
            select
                file_name, requirements,
                file_id, schematic_id
            from 
                files
            where
                schematic_id = $1
            "#,
            schematic_id
        )
        .fetch_all(&ctx.pool)
        .await?;

        Ok(Json(files))
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

        let dependencies: Vec<String> = vec![]; // todo: extract dependencies from file

        sqlx::query!(
            r#"
            insert into files (
                file_name, schematic_id,
                requirements
            )
            values (
                $1, $2, $3
            )
            "#,
            file_name,
            schematic_id,
            &dependencies[..]
        )
        .execute(&mut *transaction)
        .await
        .on_constraint("files_file_name_schematic_id_key", |_| ApiError::BadRequest)?;

        let location = storage::schematic_file_path(&schematic_id);

        storage::upload::save_schematic(&location, &file_name, &form.file.contents)?;
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
    #[oai(path = "/files/:file_id", method = "delete")]
    async fn remove_file_by_id(
        &self,
        Data(ctx): Data<&ApiContext>,   
        Path(file_id): Path<Uuid>,
        session: Session,
    ) -> ApiResult<()> {
        let mut transaction = ctx.pool.begin().await?;
        
        let schematic_meta = sqlx::query!(
            r#"
            select author 
            from schematics 
            where schematic_id = (select schematic_id from files where file_id = $1)
            "#,
            file_id
        )
        .fetch_optional(&mut *transaction)
        .await?
        .ok_or(ApiError::NotFound)?;

        if schematic_meta.author != session.user_id() &&
                !session.is_moderator(&mut *transaction).await? {
            return Err(ApiError::Unauthorized);
        }

        let file_meta = sqlx::query!(
            r#"
            delete from files
            where file_id = $1
            returning schematic_id, file_name
            "#,
            file_id
        )
        .fetch_one(&mut *transaction)
        .await?;
    
        let path = storage::schematic_file_path(&file_meta.schematic_id);
        
        // Remove the file last since it's the hardest part to rollback if something
        // else goes wrong
        tokio::fs::remove_file(path.join(file_meta.file_name))
            .await
            .map_err(anyhow::Error::new)?;
    
        transaction.commit().await?;
    
        Ok(())
    }   
}