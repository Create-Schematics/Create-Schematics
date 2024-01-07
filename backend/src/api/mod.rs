use std::net::SocketAddr;

use clap::Args;
use reqwest::Method;
use sqlx::PgPool;
use poem::endpoint::StaticFilesEndpoint;
use poem::http::header;
use poem::listener::TcpListener;
use poem::{Route, Server, EndpointExt};
use poem::middleware::{Cors, CookieJarManager};
use poem_openapi::{LicenseObject, ContactObject, OpenApiService, OpenApi};

use crate::database::postgres;
use crate::database::postgres::DatabaseArguments;
use crate::database::redis;
use crate::database::redis::{RedisPool, RedisArguments};
use crate::middleware::logging::middleware_log;

pub mod auth;
pub mod v1;

pub mod openapi;

#[derive(Args, Debug)]
pub struct StartCommandServerArguments {
    #[arg(help = "The hostname or ip address to listen for connections on")]
    #[arg(env = "BIND_ADDRESS", short = 'b', long = "bind")]
    #[arg(default_value = "0.0.0.0:3000")]
    pub listen_address: SocketAddr,

    #[command(next_help_heading = "Redis")]
    #[command(flatten)]
    pub redis: RedisArguments,

    #[command(next_help_heading = "Database")]
    #[command(flatten)]
    pub postgres: DatabaseArguments,
}

#[derive(Clone)]
pub struct ApiContext {
    pub pool: PgPool,
    pub redis_pool: RedisPool
}

pub fn configure() -> impl OpenApi {
    (auth::configure(), v1::configure())
} 

pub fn build_openapi_service() -> OpenApiService<impl OpenApi, ()> {
    let apis = configure();
    
    let license = LicenseObject::new("MIT")
        .url("https://github.com/Create-Schematics/Create-Schematics/blob/master/LICENSE");

    let contact = ContactObject::new()
        .name("Create-Schematics")
        .url("https://github.com/Create-Schematics");

    OpenApiService::new(apis, "Create Schematics REST API", "0.1")
        .server("/api")
        .license(license)
        .contact(contact)
        .external_document("https://github.com/Create-Schematics/Create-Schematics")
}

pub async fn serve(
    StartCommandServerArguments {
        listen_address,
        redis,
        postgres,
        ..
    }: StartCommandServerArguments,
) -> Result<(), anyhow::Error> {
    let pool = postgres::connect(postgres).await?;
    let redis_pool = redis::connect(redis).await?;

    let api_service = build_openapi_service();

    let swagger = api_service.swagger_ui();

    let json_spec = api_service.spec_endpoint();
    let yaml_spec = api_service.spec_endpoint_yaml();

    let app = Route::new()
        .nest("/api", Route::new()
            .nest("/", api_service)
            .nest("/swagger-ui", swagger)
            .at("/openapi.json", json_spec)
            .at("/openapi.yaml", yaml_spec)
        )
        .nest("/upload", StaticFilesEndpoint::new("./static/upload"))
        .with(Cors::new()
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
            .allow_credentials(true)
            .max_age(86400)
        )
        .with(CookieJarManager::new())
        .around(middleware_log)
        .data(ApiContext { pool, redis_pool });

    Server::new(TcpListener::bind(listen_address))
        .run(app)
        .await?;

    Ok(())
}