use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

use crate::cli::server::StartCommandDatabaseArguments;

pub async fn connect(
    StartCommandDatabaseArguments {
        database_url,
        min_connections,
        max_connections,
    }: StartCommandDatabaseArguments,
) -> Result<PgPool, sqlx::error::Error> {
    PgPoolOptions::new()
        .min_connections(min_connections)
        .max_connections(max_connections)
        .connect(&database_url)
        .await
}
