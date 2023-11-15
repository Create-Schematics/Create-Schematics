use axum::Router;

use crate::cli::server::ApiContext;

pub mod v1;

pub fn configure() -> Router<ApiContext> {
    Router::new()
}