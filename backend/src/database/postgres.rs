use clap::Args;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

#[derive(Args, Debug)]
pub struct DatabaseArguments {
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

pub async fn connect(
    DatabaseArguments {
        database_url,
        min_connections,
        max_connections,
    }: DatabaseArguments,
) -> Result<PgPool, sqlx::error::Error> {
    PgPoolOptions::new()
        .min_connections(min_connections)
        .max_connections(max_connections)
        .connect(&database_url)
        .await
}
