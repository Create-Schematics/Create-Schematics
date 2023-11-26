use axum::Router;
use axum::routing::post;
use axum::Json;
use axum::extract::State;
use tower_cookies::Cookies;
use utoipa::ToSchema;

use crate::authentication::password::{hash_password_argon2, verify_password_argon2};
use crate::authentication::session::Session;
use crate::response::ApiResult;
use crate::error::{ApiError, ResultExt};
use crate::models::user::{User, Permissions};
use crate::api::ApiContext;

#[derive(Debug, Deserialize, ToSchema)]
pub (in crate::api) struct Login {
    /// The username or email of the account to log into
    /// 
    #[schema(example="My username")]
    #[schema(min_length=3, max_length=20)]
    username: String,

    #[schema(example="My password")]
    #[schema(min_length=8)]
    password: String
}

#[derive(Debug, Deserialize, ToSchema)]
pub (in crate::api) struct Signup {
    #[schema(example="My username")]
    #[schema(min_length=3, max_length=20)]
    username: String,

    #[schema(example="email@email.com")]
    email: String,

    #[schema(example="My password")]
    password: String
}

#[derive(Debug, Deserialize, ToSchema)]
pub (in crate::api) struct UpdateUser {
    #[schema(example="My new username")]
    #[schema(min_length=3, max_length=20)]
    username: Option<String>,
    
    #[schema(example="My new password")]
    password: Option<String>
}

pub (in crate::api::v1) fn configure() -> Router<ApiContext> {
    Router::new()
        .route(
            "/users",
            post(signup)
            .patch(update_current_user)
            .get(current_user) 
            .delete(remove_current_user)
        )
        .route(
            "/users/login", 
            post(login)
        )
        .route(
            "/users/logout", 
            post(logout)
        )
}

#[utoipa::path(
    get,
    path = "/users",
    context_path = "/api/v1",
    tag = "v1",
    responses(
        (status = 200, description = "Successfully found current users", body = User, content_type = "application/json"),
        (status = 401, description = "You must be logged in"),
        (status = 500, description = "An error occurred while authenticating the user")
    ),
    security(("session_cookie" = []))
)]
async fn current_user(
    State(ctx): State<ApiContext>,
    session: Session
) -> ApiResult<Json<User>> {
    sqlx::query_as!(
        User,
        r#"
        select user_id, username,
               permissions, email, 
               password_hash
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
    post,
    path = "/users",
    context_path = "/api/v1",
    tag = "v1",
    request_body(
        content = Signup, description = "Information about the new user", content_type = "application/json"
    ),
    responses(
        (status = 200, description = "Successfully signed up in", body = User, content_type = "application/json"),
        (status = 429, description = "Either the username or password is already used"),
        (status = 500, description = "An error occurred while authenticating the user")
    ),
    security(())
)]
async fn signup(
    State(ctx): State<ApiContext>,
    cookies: Cookies,
    Json(form): Json<Signup>
) -> ApiResult<Json<User>> {
    let password_hash = hash_password_argon2(form.password).await?;
    let permissions = Permissions::default().bits() as i32;

    let user = sqlx::query_as!(
        User,
        r#"
        insert into users 
            (username, email, permissions, password_hash)
        values 
            ($1, $2, $3, $4)
        returning
            user_id, username, email,
            permissions, password_hash
        "#,
        form.username,
        form.email,
        permissions,
        password_hash
    )
    .fetch_one(&ctx.pool)
    .await
    .on_constraint("users_username_key", |_| {
        ApiError::unprocessable_entity([("username", "username taken")])
    })
    .on_constraint("users_email_key", |_| {
        ApiError::unprocessable_entity([("email", "email taken")])
    })?;

    let session = Session::new_for_user(user.user_id);

    session.save(&ctx.redis_pool).await?;

    cookies.add(session.into_cookie());

    Ok(Json(user))
}

#[utoipa::path(
    post,
    path = "/users/login",
    context_path = "/api/v1",
    tag = "v1",
    request_body(
        content = Login, description = "Login information with either the username or email", content_type = "application/json"
    ),
    responses(
        (status = 200, description = "Successfully logged in", body = User, content_type = "application/json"),
        (status = 401, description = "Invalid login credentials"),
        (status = 500, description = "An error occurred while authenticating user")
    ),
    security(())
)]
async fn login(
    State(ctx): State<ApiContext>,
    cookies: Cookies,
    Json(form): Json<Login>
) -> ApiResult<Json<User>> {
    let user = sqlx::query_as!(
        User,
        r#"
        select user_id, username, 
               permissions, email, 
               password_hash
        from users 
        where username = $1
        or email = $1
        "#,
        form.username
    )
    .fetch_optional(&ctx.pool)
    .await?
    .ok_or(ApiError::NotFound)?;

    verify_password_argon2(form.password, &user.password_hash).await?;

    let session = Session::new_for_user(user.user_id);

    session.save(&ctx.redis_pool).await?;

    cookies.add(session.into_cookie());

    Ok(Json(user))
}

#[utoipa::path(
    patch,
    path = "/schematics/{id}",
    context_path = "/api/v1",
    tag = "v1",
    request_body(
        content = UpdateUser, description = "The values to update", content_type = "application/json"
    ),
    responses(
        (status = 200, description = "Successfully updated the schematic", body = Schematic, content_type = "application/json"),
        (status = 401, description = "You need to be logged in to update your profile"),
        (status = 422, description = "An account witht that username already exists"),
        (status = 500, description = "An internal server error occurred")
    ),
    security(("session_cookie" = []))
)]
async fn update_current_user(
    State(ctx): State<ApiContext>,
    cookies: Cookies,
    session: Session,
    Json(form): Json<UpdateUser>
) -> ApiResult<Json<User>> {
    let mut transaction = ctx.pool.begin().await?;

    let password_hash = match form.password {
        Some(password) => Some(hash_password_argon2(password).await?),
        None => None
    };

    let user = sqlx::query_as!(
        User,
        r#"
        update users
            set 
                username = coalesce($1, username),
                password_hash = coalesce($2, password_hash)
            where user_id = $3
            returning
                user_id,
                username,
                email,
                permissions,
                password_hash
        "#,
        form.username,
        password_hash,
        session.user_id
    )
    .fetch_optional(&mut *transaction)
    .await
    .on_constraint("users_username_key", |_| {
        ApiError::unprocessable_entity([("username", "username taken")])
    })?
    .ok_or(ApiError::NotFound)?;
    
    transaction.commit().await?;

    let session = Session::new_for_user(user.user_id);
    session.save(&ctx.redis_pool).await?;

    cookies.add(session.into_cookie());

    Ok(Json(user))
}

#[utoipa::path(
    post,
    path = "/users/logout",
    context_path = "/api/v1",
    tag = "v1",
    responses(
        (status = 200, description = "Successfully logged out"),
        (status = 401, description = "You must be logged in to lout out"),
        (status = 500, description = "An error occurred while authenticating user")
    ),
    security(("session_cookie" = []))
)]
async fn logout(
    State(ctx): State<ApiContext>,
    session: Session,
    cookies: Cookies
) -> ApiResult<()> {
    Session::take_from_jar(&cookies);

    session.clear(&ctx.redis_pool).await?;

    Ok(())
}

#[utoipa::path(
    post,
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