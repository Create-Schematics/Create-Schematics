use poem::web::Data;
use poem_openapi::param::{Path, Query};
use poem_openapi::payload::Json;
use poem_openapi_derive::{Object, OpenApi};
use time::OffsetDateTime;
use uuid::Uuid;

use crate::authentication::schemes::Session;
use crate::error::{ApiError, ResultExt};
use crate::models::schematic::Schematic;
use crate::models::user::{User, Role};
use crate::response::ApiResult;
use crate::api::ApiContext;

pub struct UsersApi;

#[derive(Debug, Deserialize, Object)]
pub struct UpdateUser {
    #[oai(validator(min_length=3, max_length=30))]
    pub username: Option<String>,
    #[oai(validator(min_length=3, max_length=30))]
    pub displayname: Option<String>,
    #[oai(validator(max_length=256))]
    pub about: Option<String>,
    pub avatar_url: Option<String>,
}

#[derive(Debug, Serialize, Object)]
pub struct CurrentUser {
    pub user_id: Uuid,
    #[oai(validator(min_length=3, max_length=30))]
    pub username: String,
    #[oai(validator(min_length=3, max_length=30))]
    pub displayname: String,
    pub avatar: Option<String>,
    #[oai(validator(max_length=256))]
    pub about: Option<String>,
    pub role: Role,
    pub email: Option<String>,
    pub updated_at: Option<OffsetDateTime>,
    pub created_at: OffsetDateTime
}

#[OpenApi(prefix_path="/v1")]
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
            select 
                user_id, displayname,
                username, email, about, 
                role, updated_at, avatar, 
                created_at
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

    /// Fetches a user by their username, for privacy their email will not be included
    /// 
    #[oai(path="/users/:username", method = "get")]
    async fn fetch_user_by_id(
        &self,
        Data(ctx): Data<&ApiContext>,
        Path(username): Path<String>
    ) -> ApiResult<Json<User>> {
        sqlx::query_as!(
            User,
            r#"
            select 
                user_id, username,
                displayname, role,
                avatar, about,
                created_at, updated_at
            from users
            where username = $1
            "#,
            username
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
    #[oai(path="/users/:username/schematics", method = "get")]
    async fn get_uploaded_schematics(
        &self,
        Data(ctx): Data<&ApiContext>,
        Path(username): Path<String>,
        Query(limit): Query<Option<i64>>,
        Query(offset): Query<Option<i64>>,
    ) -> ApiResult<Json<Vec<Schematic>>> {
        let schematics = sqlx::query_as!(
            Schematic,
            r#"
            select 
                schematic_id, schematic_name, body, 
                images, author, downloads,
                create_version_id, game_version_id,
                created_at, updated_at
            from 
                schematics
            where 
                author = (select user_id from users where username = $1)
            limit $2 offset $3
            "#,
            username,
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
                    displayname = coalesce($2, displayname),
                    about = coalesce($3, about),
                    avatar = coalesce($4, avatar)
                where 
                    user_id = $5
                returning
                    user_id,
                    username,
                    displayname,
                    about,
                    email,
                    role,
                    avatar,
                    created_at,
                    updated_at
            "#,
            form.username,
            form.displayname,
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
