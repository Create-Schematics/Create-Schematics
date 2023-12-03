use std::time::Duration;
use std::net::SocketAddr;

use axum::Router;
use axum::http::header;
use axum::http::Method;
use clap::Args;
use sqlx::PgPool;
use tower_cookies::CookieManagerLayer;
use tower_http::trace::TraceLayer;
use tower_http::cors::{Any, CorsLayer};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::database::postgres;
use crate::database::postgres::StartCommandDatabaseArguments;
use crate::database::redis;
use crate::database::redis::{RedisPool, StartCommandRedisArguments};

use self::auth::StartCommandOauthArguments;

pub mod auth;
pub mod v1;

pub mod openapi;

pub fn configure() -> Router<ApiContext> {
    Router::new()
        .nest("/v1", v1::configure())
}

#[derive(Args, Debug)]
pub struct StartCommandServerArguments {
    #[arg(help = "The hostname or ip address to listen for connections on")]
    #[arg(env = "BIND_ADDRESS", short = 'b', long = "bind")]
    #[arg(default_value = "0.0.0.0:3000")]
    pub listen_address: SocketAddr,

    #[command(next_help_heading = "Database")]
    #[command(flatten)]
    pub postgres: StartCommandDatabaseArguments,

    #[command(next_help_heading = "Redis")]
    #[command(flatten)]
    pub redis: StartCommandRedisArguments,

    #[command(next_help_heading = "Oauth")]
    #[command(flatten)]
    pub oauth: StartCommandOauthArguments,
}

#[derive(Clone)]
pub struct ApiContext {
    pub pool: PgPool,
    pub redis_pool: RedisPool
}

pub async fn init(
    StartCommandServerArguments {
        listen_address,
        postgres,
        redis,
        oauth,
        ..
    }: StartCommandServerArguments,
) -> Result<(), anyhow::Error> {
    let pool = postgres::connect(postgres).await?;
    let redis_pool = redis::connect(redis)?;

    let app = Router::new()
        .nest("/api", Router::new()
            .merge(crate::api::configure())
            .merge(crate::api::auth::configure(oauth)?)
        )
        .merge(SwaggerUi::new("/swagger-ui").url("/openapi.json", openapi::ApiDoc::openapi()))
        .layer(CorsLayer::default()
            .allow_headers([
                header::AUTHORIZATION,
                header::WWW_AUTHENTICATE,
                header::CONTENT_TYPE,
                header::ORIGIN,
                header::COOKIE,
            ])
            .allow_methods([
                Method::GET,
                Method::PUT,
                Method::POST,
                Method::PUT,
                Method::DELETE,
                Method::PATCH,
                Method::OPTIONS
            ])
            .allow_origin(Any)
            .max_age(Duration::from_secs(86400))
        )
        .layer(CookieManagerLayer::new())
        .layer(TraceLayer::new_for_http())
        .with_state(ApiContext { pool, redis_pool });

    tracing::info!("Listening on http://{}", listen_address);
    
    axum::Server::bind(&listen_address)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}