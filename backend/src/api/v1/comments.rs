use poem::web::Data;
use poem_openapi::OpenApi;
use poem_openapi::param::{Path, Query};
use poem_openapi::payload::Json;
use poem_openapi_derive::{Object, Multipart};
use uuid::Uuid;

use crate::api::ApiContext;
use crate::middleware::validators::Profanity;
use crate::response::ApiResult;
use crate::models::comment::Comment;
use crate::error::ApiError;
use crate::authentication::schemes::Session;

pub (in crate::api::v1) struct CommentsApi;

#[derive(Serialize, Debug, Object)]
pub (in crate::api::v1) struct FullComment {
    pub comment_id: Uuid,
    pub parent: Option<Uuid>,
    pub comment_author: Uuid,
    pub comment_body: String,
    pub schematic_id: String,
    pub author_username: String
}

#[derive(Multipart, Debug)]
pub (in crate::api::v1) struct CommentBuilder {
    #[oai(validator(max_length=1024, custom="Profanity"))]
    pub comment_body: String,
    pub parent: Option<Uuid>
}

#[derive(Multipart, Debug)]
pub (in crate::api::v1) struct UpdateComment {
    #[oai(validator(max_length=1024, custom="Profanity"))]
    pub comment_body: Option<String>
}

#[OpenApi(prefix_path="/v1")]
impl CommentsApi {

    /// Fetches a number of the comments on a schematic as well as some basic
    /// additional information about their author such as their avatar url
    /// and usesrname to prevent the need for subsequent requests. By default
    /// if no limit for comments is set then up to 20 will be returned at a
    /// time.
    /// 
    /// Note that comment bodies can contain markdown which will need to be 
    /// handled accordingly
    /// 
    #[oai(path = "/schematics/:schematic_id/comments", method = "get")]
    async fn get_comments_by_schematic(
        &self,
        Data(ctx): Data<&ApiContext>,        
        Path(schematic_id): Path<Uuid>,
        Query(limit): Query<Option<i64>>,
        Query(offset): Query<Option<i64>>,
    ) -> ApiResult<Json<Vec<FullComment>>> {
        let schematics = sqlx::query_as!(
            FullComment,
            r#"
            select 
                comment_id, comment_author,
                comment_body, schematic_id,
                username as author_username,
                parent
            from 
                comments
                inner join users on comment_author = user_id
            where 
                schematic_id = $1
            order by 
                parent
            limit $2 
            offset $3
            "#,
            schematic_id,
            limit.unwrap_or(20),
            offset.unwrap_or(0)
        )
        .fetch_all(&ctx.pool)
        .await?;
    
        Ok(Json(schematics))
    }   
    
    /// Uploads a comment to a given schematic for the current user returning
    /// information about the new comment including its id. 
    /// 
    /// The comments body can contain markdown which will be sanitized
    /// accordingly, however it cannot contain profanity wich will result in
    /// a `422 Conflict` being returned. 
    /// 
    /// If you believe something is being falsely flagged as profanity please
    /// contact us either on github or through other chanels provided in the 
    /// openapi spec.
    /// 
    #[oai(path = "/schematics/:schematic_id/comments", method = "post")]
    async fn post_comment(
        &self,
        Data(ctx): Data<&ApiContext>,  
        Path(schematic_id): Path<Uuid>,
        Session(user_id): Session,
        form: CommentBuilder
    ) -> ApiResult<Json<Comment>> {
        let mut transaction = ctx.pool.begin().await?;

        // Check that the parent comment both exists and is on this schematic
        if let Some(parent_id) = form.parent {
            let parent_meta = sqlx::query!(
                r#"select schematic_id from comments where comment_id = $1"#,
                parent_id
            )
            .fetch_optional(&mut *transaction)
            .await?
            .ok_or(ApiError::NotFound)?;

            if parent_meta.schematic_id != schematic_id {
                return Err(ApiError::BadRequest);
            }
        } 

        let comment = sqlx::query_as!(
            Comment,
            r#"
            insert into comments (
                comment_author, comment_body,
                parent, schematic_id
            )
            values (
                $1, $2, $3, $4
            )
            returning
                comment_id,
                parent,
                comment_author,
                comment_body,
                schematic_id
            "#,
            user_id,
            form.comment_body,
            form.parent,
            schematic_id
        )
        .fetch_one(&mut *transaction)
        .await?;

        transaction.commit().await?;

        Ok(Json(comment))
    }

    /// Fetches a specific comment by it's id aswell as some additional
    /// information about it's author such as their username and avatar url
    /// to avoid subsequent requests. 
    /// 
    /// Note the comemnts body can contain markdown which will need to be
    /// displayed accordingly to the user
    /// 
    /// If you are looking to fetch comments from a schematic see 
    /// `GET /schematics/:id/comments`
    /// 
    #[oai(path = "/comments/:comment_id", method = "get")]
    async fn get_comment_by_id(
        &self,
        Data(ctx): Data<&ApiContext>,  
        Path(comment_id): Path<Uuid>,
    ) -> ApiResult<Json<FullComment>> {
        sqlx::query_as!(
            FullComment,
            r#"
            select 
                comment_id, comment_author,
                comment_body, schematic_id,
                username as author_username,
                parent
            from 
                comments
                inner join users on comment_author = user_id
            where 
                comment_id = $1
            "#,
            comment_id,
        )
        .fetch_optional(&ctx.pool)
        .await?
        .ok_or(ApiError::NotFound)
        .map(Json)
    }   
    
    /// Updates a given comment by its id, all fields are optional but at least
    /// one is required to be present. 
    /// 
    /// The new body can contain markdown but not profanity, if it is detected
    /// to be innapropriate then the reqeust will be denied with `422 Unprocessable
    /// Entity`
    /// 
    /// The current user must also own the comment even if they have permission to
    /// moderate comments
    /// 
    #[oai(path = "/comments/:comment_id", method = "patch")]
    async fn update_comment_by_id(
        &self,
        Data(ctx): Data<&ApiContext>, 
        Path(comment_id): Path<Uuid>,
        Session(user_id): Session,
        form: UpdateComment
    ) -> ApiResult<Json<Comment>> {
        let mut transaction = ctx.pool.begin().await?;
        
        let user_meta = sqlx::query!(
            r#"select comment_author from comments where comment_id = $1"#,
            comment_id
        )
        .fetch_optional(&mut *transaction)
        .await?
        .ok_or(ApiError::NotFound)?;

        if user_meta.comment_author != user_id {
            return Err(ApiError::Forbidden)
        }
    
        let comment = sqlx::query_as!(
            Comment,
            r#"
            update comments
                set 
                    comment_body = coalesce($1, comment_body)
                where 
                    comment_id = $2
                returning
                    comment_id,
                    parent,
                    comment_author,
                    comment_body,
                    schematic_id
            "#,
            form.comment_body,
            comment_id
        )
        .fetch_optional(&mut *transaction)
        .await?
        .ok_or(ApiError::NotFound)?;
    
        transaction.commit().await?;
    
        Ok(Json(comment))
    }
    
    /// Removes a comment from a schematic by it's id, this requires for the
    /// current user to either own the comment or have permission to moderate
    /// comments
    /// 
    #[oai(path = "/comments/:comment_id", method = "delete")]
    async fn delete_comment_by_id(
        &self,
        Data(ctx): Data<&ApiContext>, 
        Path(comment_id): Path<Uuid>,
        session: Session,
    ) -> ApiResult<()> {
        let mut transaction = ctx.pool.begin().await?;
    
        let user_meta = sqlx::query!(
            r#"select comment_author from comments where comment_id = $1"#,
            comment_id
        )
        .fetch_optional(&mut *transaction)
        .await?
        .ok_or(ApiError::NotFound)?;

        if user_meta.comment_author != session.user_id()
                && !session.is_moderator(&mut *transaction).await? {
            return Err(ApiError::Forbidden)
        }

        // We dont need to validate the the comment previously existed here as that was implicitly
        // checked when ensuring the user was the author of the comment
        sqlx::query!(
            r#"
            delete from comments
            where comment_id = $1
            "#,
            comment_id
        )
        .execute(&mut *transaction)
        .await?;
    
        transaction.commit().await?;
    
        Ok(())
    }    
}