use axum::Router;

use crate::api::ApiContext;

pub mod comments;
pub mod favorites;
pub mod schematics;
pub mod users;

pub (in crate::api) fn configure() -> Router<ApiContext> {
    Router::new()
        .merge(users::configure())
        .merge(schematics::configure())
        .merge(comments::configure())
        .merge(favorites::configure())
}