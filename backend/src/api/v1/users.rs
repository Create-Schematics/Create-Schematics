use poem::web::Data;
use poem_openapi::param::{Path, Query};
use poem_openapi::payload::Json;
use poem_openapi_derive::{Object, OpenApi};
use uuid::Uuid;

use crate::authentication::session::Session;
use crate::error::{ApiError, ResultExt};
use crate::models::schematic::Schematic;
use crate::models::user::{User, Role};
use crate::response::ApiResult;
use crate::api::ApiContext;

pub struct UsersApi;

#[derive(Debug, Deserialize, Object)]
pub struct UpdateUser {
    #[oai(validator(min_length=3, max_length=30))]
    username: Option<String>,
    #[oai(validator(max_length=256))]
    about: Option<String>,
    avatar_url: Option<String>,
}

#[derive(Debug, Serialize, Object)]
pub struct CurrentUser {
    pub user_id: Uuid,
    #[oai(validator(min_length=3, max_length=30))]
    pub username: String,
    pub avatar: Option<String>,
    #[oai(validator(max_length=256))]
    pub about: Option<String>,
    pub role: Role,
    pub email: Option<String>,
}

#[OpenApi(prefix_path="/api/v1")]
impl UsersApi {

    /// Fetches information about the current user including their email
    /// 
    #[oai(path="/users", method = "get")]
    async fn current_user(
        &self,
        Data(ctx): Data<&ApiContext>,
        Session(user_id): Session
    ) -> ApiResult<Json<CurrentUser>> {
        sqlx::query_as!(
            CurrentUser,
            r#"
            select user_id, username,
                   email, about, role,
                   avatar
            from users
            where user_id = $1
            "#,
            user_id
        )
        .fetch_optional(&ctx.pool)
        .await?
        .ok_or(ApiError::Unauthorized)
        .map(Json)
    }

    /// Fetches a user by their id, for privacy their email will not be included
    /// 
    #[oai(path="/users/:id", method = "get")]
    async fn fetch_user_by_id(
        &self,
        Data(ctx): Data<&ApiContext>,
        Path(user_id): Path<Uuid>
    ) -> ApiResult<Json<User>> {
        sqlx::query_as!(
            User,
            r#"
            select user_id, username, 
                   avatar, role, about
            from users
            where user_id = $1
            "#,
            user_id
        )
        .fetch_optional(&ctx.pool)
        .await?
        .ok_or(ApiError::NotFound)
        .map(Json)
    }

    /// Fetches a number of schematics created by the specified user. User
    /// information will not be included with the schematic as it is assumed
    /// that this information is already known.
    /// 
    /// If a limit is not specified 20 will be fetched by default.
    /// 
    #[oai(path="/users/:id/schematics", method = "get")]
    async fn get_uploaded_schematics(
        &self,
        Data(ctx): Data<&ApiContext>,
        Path(user_id): Path<Uuid>,
        Query(limit): Query<Option<i64>>,
        Query(offset): Query<Option<i64>>,
    ) -> ApiResult<Json<Vec<Schematic>>> {
        let schematics = sqlx::query_as!(
            Schematic,
            r#"
            select schematic_id, schematic_name,
                   body, files, images, author,
                   create_version_id, downloads,
                   game_version_id
            from schematics
            where author = $1
            limit $2 offset $3
            "#,
            user_id,
            limit.unwrap_or(20),
            offset.unwrap_or(0)
        )
        .fetch_all(&ctx.pool)
        .await?;
    
        Ok(Json(schematics))
    }

    /// Updates information about the current user. All fields are optional but
    /// at least one is required.
    /// 
    /// All usernames must be unique, if the requested new username is already
    /// used a `422 Unprocessable Entity` error will be returned
    /// 
    #[oai(path="/users", method = "patch")]
    async fn update_current_user(
        &self,
        Data(ctx): Data<&ApiContext>,
        Session(user_id): Session,
        Json(form): Json<UpdateUser>
    ) -> ApiResult<Json<CurrentUser>> {
        let mut transaction = ctx.pool.begin().await?;
    
        let user = sqlx::query_as!(
            CurrentUser,
            r#"
            update users
                set 
                    username = coalesce($1, username),
                    about = coalesce($2, about),
                    avatar = coalesce($3, avatar)
                where 
                    user_id = $4
                returning
                    user_id,
                    username,
                    about,
                    email,
                    role,
                    avatar
            "#,
            form.username,
            form.about,
            form.avatar_url,
            user_id
        )
        .fetch_optional(&mut *transaction)
        .await
        .on_constraint("users_username_key", |_| {
            ApiError::unprocessable_entity([("username", "username taken")])
        })?
        .ok_or(ApiError::NotFound)?;
        
        transaction.commit().await?;
    
        Ok(Json(user))
    }

    /// Removes the current users account and invalidates any active sessions
    /// aswell as removing the current session from their cookies.
    /// 
    #[oai(path="/users", method = "delete")]
    async fn remove_current_user(
        &self,
        Data(ctx): Data<&ApiContext>,
        Session(user_id): Session
    ) -> ApiResult<()>  {
        let mut transaction = ctx.pool.begin().await?;

        sqlx::query!(
            r#"
            delete from users
            where user_id = $1
            "#,
            user_id
        )
        .execute(&mut *transaction)
        .await?;

        // TDOO: Handle removing session

        transaction.commit().await?;

        Ok(())
    }    
}
