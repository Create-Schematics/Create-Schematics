use axum::Router;

use crate::api::ApiContext;

pub mod comments;
pub mod favorites;
pub mod schematics;
pub mod likes;
pub mod users;
pub mod profile;
pub mod tags;
pub mod collections;
pub mod images;

pub (in crate::api) fn configure() -> Router<ApiContext> {
    Router::new()
        .merge(users::configure())
        .merge(schematics::configure())
        .merge(images::configure())
        .merge(comments::configure())
        .merge(profile::configure())
        .merge(favorites::configure())
        .merge(collections::configure())
        .merge(likes::configure())
        .merge(tags::configure())
}