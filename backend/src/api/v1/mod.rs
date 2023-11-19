use axum::Router;

use crate::cli::server::ApiContext;

pub mod schematics;
pub mod users;

pub (in crate::api) fn configure() -> Router<ApiContext> {
    Router::new()
        .merge(users::configure())
        .merge(schematics::configure())
}