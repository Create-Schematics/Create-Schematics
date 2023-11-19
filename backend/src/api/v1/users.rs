use axum::Router;
use axum::routing::post;
use axum::{extract::State, Json};
use tower_cookies::Cookies;
use utoipa::ToSchema;

use crate::authentication::password::{hash_password_argon2, verify_password_argon2};
use crate::authentication::session::Session;
use crate::{response::ApiResult, cli::server::ApiContext};
use crate::error::{ApiError, ResultExt};
use crate::models::user::User;

#[derive(Debug, Deserialize, ToSchema)]
pub (in crate::api) struct Login {
    username: String,
    password: String
}

#[derive(Debug, Deserialize, ToSchema)]
pub (in crate::api) struct Signup {
    username: String,
    email: String,
    password: String
}

pub (in crate::api::v1) fn configure() -> Router<ApiContext> {
    Router::new()
        .route(
            "/users",
            post(signup)
            .get(current_user) 
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
    post,
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
               email, password_hash
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

    let user = sqlx::query_as!(
        User,
        r#"
        insert into users 
            (username, email, password_hash)
        values 
            ($1, $2, $3)
        returning
            user_id, username,
            email, password_hash
        "#,
        form.username,
        form.email,
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

    session.save(ctx.redis_pool).await?;

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
               email, password_hash
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

    session.save(ctx.redis_pool).await?;

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
    Session::take_from_jar(cookies);

    session.clear(ctx.redis_pool).await?;

    Ok(())
}
