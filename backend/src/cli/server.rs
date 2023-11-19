use std::{net::SocketAddr, time::Duration};

use axum::Router;
use axum::http::{Method, header};
use tower_cookies::CookieManagerLayer;
use tower_http::cors::{CorsLayer, Any};
use tower_http::trace::TraceLayer;

use clap::Args;
use sqlx::PgPool;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::api::ApiDoc;
use crate::database::{postgres, redis};
use crate::database::redis::RedisPool;

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

#[derive(Args, Debug)]
pub struct StartCommandDatabaseArguments {
    #[arg(help = "The location of your postgres database")]
    #[arg(env = "DATABASE_URL", short = 'd', long = "database_url")]
    #[arg(default_value = "postgresql://localhost")]
    pub database_url: String,

    #[arg(help = "The minimum number of connections to the database")]
    #[arg(env = "DATABASE_MIN_CONNECTIONS", long = "database_min_connections")]
    #[arg(default_value = "0")]
    pub min_connections: u32,

    #[arg(help = "The maximum number of connections to the database")]
    #[arg(env = "DATABASE_MAX_CONNECTIONS", long = "database_max_pconnections")]
    #[arg(default_value = "50")]
    pub max_connections: u32,
}

#[derive(Args, Debug)]
pub struct StartCommandRedisArguments {
    #[arg(help = "The location of your redis instance")]
    #[arg(env = "REDIS_URL", short = 'r', long = "redis_url")]
    #[arg(default_value = "redis://localhost")]
    pub redis_url: String,
    
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
        ..
    }: StartCommandServerArguments,
) -> Result<(), anyhow::Error> {
    let database_pool = postgres::connect(postgres).await?;
    let redis_pool = redis::connect(redis)?;

    let app = Router::new()
        .nest("/api", crate::api::configure())
        .merge(SwaggerUi::new("/swagger-ui").url("/openapi.json", ApiDoc::openapi()))
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
                Method::DELETE,
                Method::OPTIONS
            ])
            .allow_origin(Any)
            .max_age(Duration::from_secs(86400))
        )
        .layer(CookieManagerLayer::new())
        .layer(TraceLayer::new_for_http())
        .with_state(ApiContext { pool: database_pool, redis_pool });
    
    axum::Server::bind(&listen_address)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

