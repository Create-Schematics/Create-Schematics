use std::{net::SocketAddr, time::Duration};

use axum::Router;
use axum::http::{Method, header};
use tower_http::cors::{CorsLayer, Any};
use tower_http::trace::TraceLayer;

use clap::Args;
use sqlx::PgPool;

#[derive(Args, Debug)]
pub struct StartCommandServerArguments {
    #[arg(help = "The hostname or ip address to listen for connections on")]
    #[arg(env = "BIND_ADDR", short = 'b', long = "bind")]
    #[arg(default_value = "0.0.0.0:3000")]
    pub listen_address: SocketAddr,

    #[command(flatten)]
    #[command(next_help_heading = "Database")]
    pub database: StartCommandDabaseArguments,
}

#[derive(Args, Debug)]
pub struct StartCommandDabaseArguments {
    #[arg(help = "The location of your postgres database")]
    #[arg(env = "DATABASE_URL", short = 'd', long = "database_url")]
    #[arg(default_value = "postgresql://localhost")]
    pub database_url: String,

    #[arg(help = "The minimum number of connections to the database")]
    #[arg(env = "MIN_CONNECTIONS", short = 'm', long = "min_connections")]
    #[arg(default_value = "0")]
    pub min_connections: u32,

    #[arg(help = "The maximum number of connections to the database")]
    #[arg(env = "MAX_CONNECTIONS", short = 'u', long = "max_connections")]
    #[arg(default_value = "50")]
    pub max_connections: u32,
}

#[derive(Clone)]
pub struct ApiContext {
    pub pool: PgPool
}

pub async fn init(
    StartCommandServerArguments {
        listen_address,
        database,
        ..
    }: StartCommandServerArguments,
) -> Result<(), anyhow::Error> {
    let pool = crate::database::postgres::connect(database).await?;

    let app = Router::new()
        .nest("/api", crate::api::configure())
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
        .layer(TraceLayer::new_for_http())
        .with_state(ApiContext { pool });
    
    axum::Server::bind(&listen_address)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
