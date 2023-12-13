use axum::Router;

use crate::api::ApiContext;

pub mod comments;
pub mod schematics;
pub mod likes;
pub mod users;
pub mod profile;
pub mod tags;
pub mod collections;
pub mod images;
pub mod files;

pub (in crate::api) fn configure() -> Router<ApiContext> {
    Router::new()
        .merge(users::configure())
        .merge(schematics::configure())
        .merge(images::configure())
        .merge(files::configure())
        .merge(comments::configure())
        .merge(profile::configure())
        .merge(collections::configure())
        .merge(likes::configure())
        .merge(tags::configure())
}