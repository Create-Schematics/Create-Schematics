use std::{net::SocketAddr, time::Duration};

use axum::{Router, http::{header, Method}};
use clap::Args;
use sqlx::PgPool;
use tower_cookies::CookieManagerLayer;
use tower_http::{cors::{Any, CorsLayer}, trace::TraceLayer};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::database::postgres::{self, StartCommandDatabaseArguments};
use crate::database::redis::{self, RedisPool, StartCommandRedisArguments};

pub mod v1;

pub mod openapi;

pub fn configure() -> Router<ApiContext> {
    Router::new()
        .nest("/v1", v1::configure())
}

#[derive(Args, Debug)]
pub struct StartCommandServerArguments {
    #[arg(help = "The hostname or ip address to listen for connections on")]
    #[arg(env = "BIND_ADDR", short = 'b', long = "bind")]
    #[arg(default_value = "0.0.0.0:3000")]
    pub listen_address: SocketAddr,

    #[command(next_help_heading = "Database")]
    #[command(flatten)]
    pub postgres: StartCommandDatabaseArguments,

    #[command(next_help_heading = "Redis")]
    #[command(flatten)]
    pub redis: StartCommandRedisArguments,
}

#[derive(Clone)]
pub struct ApiContext {
    pub pool: PgPool,
    pub redis_pool: RedisPool
}

fn build_router(
    redis_pool: RedisPool,
    pool: PgPool,
) -> Router {
    Router::new()
        .nest("/api", crate::api::configure())
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
        .with_state(ApiContext { pool, redis_pool })
}

pub async fn init(
    StartCommandServerArguments {
        listen_address,
        postgres,
        redis,
        ..
    }: StartCommandServerArguments,
) -> Result<(), anyhow::Error> {
    let database_pool = postgres::connect(postgres).await?;
    let redis_pool = redis::connect(redis)?;

    let app = build_router(redis_pool, database_pool);
    
    axum::Server::bind(&listen_address)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
