use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

use crate::cli::server::StartCommandDabaseArguments;

pub async fn connect(
    StartCommandDabaseArguments {
        database_url,
        min_connections,
        max_connections,
    }: StartCommandDabaseArguments,
) -> Result<PgPool, sqlx::error::Error> {
    PgPoolOptions::new()
        .min_connections(min_connections)
        .max_connections(max_connections)
        .connect(&database_url)
        .await
}
