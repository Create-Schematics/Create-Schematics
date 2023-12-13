use axum::Router;
use axum::routing::get;
use axum::Json;
use axum::extract::{State, Path};
use tower_cookies::Cookies;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::authentication::session::Session;
use crate::error::{ApiError, ResultExt};
use crate::models::user::{User, Permissions};
use crate::response::ApiResult;
use crate::api::ApiContext;

#[derive(Debug, Deserialize, ToSchema)]
pub (in crate::api) struct UpdateUser {
    #[schema(example="My new username")]
    #[schema(min_length=3, max_length=20)]
    username: Option<String>,

    #[schema(example="About me")]
    about: Option<String>,

    #[schema(example="https://example.com/avatar.png")]
    avatar_url: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct CurrentUser {
    pub user_id: Uuid,

    #[schema(example="My username")]
    #[schema(min_length=3, max_length=20)]
    pub username: String,

    #[schema(example="https://example.com/avatar.png")]
    pub avatar: Option<String>,

    #[schema(example="Hello world")]
    pub about: Option<String>,

    #[schema(value_type=u64, example=7)]
    pub permissions: Permissions,

    #[schema(example="email@email.com")]
    pub email: String,
}

pub (in crate::api::v1) fn configure() -> Router<ApiContext> {
    Router::new()
        .route(
            "/users",
            get(current_user)
            .patch(update_current_user) 
            .delete(remove_current_user)
        )
        .route(
            "/users/:id",
            get(fetch_user_by_id)
        )
}

#[utoipa::path(
    get,
    path = "/users",
    context_path = "/api/v1",
    tag = "v1",
    responses(
        (status = 200, description = "Successfully found current users", body = CurrentUser, content_type = "application/json"),
        (status = 401, description = "You must be logged in"),
        (status = 500, description = "An error occurred while authenticating the user")
    ),
    security(("session_cookie" = []))
)]
async fn current_user(
    State(ctx): State<ApiContext>,
    session: Session
) -> ApiResult<Json<CurrentUser>> {
    sqlx::query_as!(
        CurrentUser,
        r#"
        select user_id, username, 
               email, permissions,
               avatar, about
        from users
        where user_id = $1
        "#,
        session.user_id
    )
    .fetch_optional(&ctx.pool)
    .await?
    .ok_or(ApiError::Unauthorized)
    .map(Json)
}

#[utoipa::path(
    get,
    path = "/users",
    context_path = "/api/v1/{user_id}",
    tag = "v1",
    params(
        ("user_id" = Uuid, Path, description = "The id of the user to fetch")
    ),
    responses(
        (status = 200, description = "Successfully found current users", body = User, content_type = "application/json"),
        (status = 401, description = "You must be logged in"),
        (status = 500, description = "An error occurred while authenticating the user")
    ),
    security(("session_cookie" = []))
)]
async fn fetch_user_by_id(
    State(ctx): State<ApiContext>,
    Path(user_id): Path<Uuid>
) -> ApiResult<Json<User>> {
    sqlx::query_as!(
        User,
        r#"
        select user_id, username, 
               permissions, about,
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

#[utoipa::path(
    patch,
    path = "/schematics/{schematic_id}",
    context_path = "/api/v1",
    tag = "v1",
    request_body(
        content = UpdateUser, description = "The values to update", content_type = "application/json"
    ),
    responses(
        (status = 200, description = "Successfully updated the user", body = CurrentUser, content_type = "application/json"),
        (status = 401, description = "You need to be logged in to update your profile"),
        (status = 422, description = "An account witht that username already exists"),
        (status = 500, description = "An internal server error occurred")
    ),
    security(("session_cookie" = []))
)]
async fn update_current_user(
    State(ctx): State<ApiContext>,
    session: Session,
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
                avatar,
                permissions
        "#,
        form.username,
        form.about,
        form.avatar_url,
        session.user_id
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

#[utoipa::path(
    delete,
    path = "/users",
    context_path = "/api/v1",
    tag = "v1",
    responses(
        (status = 200, description = "Successfully deleted current user"),
        (status = 401, description = "You must be logged in to remove your account"),
        (status = 500, description = "An error occurred removing your account")
    ),
    security(("session_cookie" = []))
)]
async fn remove_current_user(
    State(ctx): State<ApiContext>,
    session: Session,
    cookies: Cookies
) -> ApiResult<()>  {
    let mut transaction = ctx.pool.begin().await?;

    sqlx::query!(
        r#"
        delete from users
        where user_id = $1
        "#,
        session.user_id
    )
    .execute(&mut *transaction)
    .await?;

    Session::take_from_jar(&cookies);
    session.clear(&ctx.redis_pool).await?;

    transaction.commit().await?;

    Ok(())
}