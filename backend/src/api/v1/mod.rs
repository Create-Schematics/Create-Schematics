use poem_openapi::OpenApi;

use self::likes::LikesApi;
use self::files::FileApi;
use self::images::ImageApi;
use self::collections::CollectionsApi;
use self::moderation::ModerationApi;
use self::mods::ModApi;
use self::tags::TagsApi;
use self::users::UsersApi;
use self::schematics::SchematicsApi;
use self::comments::CommentsApi;

pub mod comments;
pub mod schematics;
pub mod likes;
pub mod users;
pub mod tags;
pub mod collections;
pub mod images;
pub mod files;
pub mod mods;
pub mod moderation;

pub fn configure() -> impl OpenApi {
    (
        UsersApi, 
        SchematicsApi, 
        LikesApi, 
        CommentsApi, 
        FileApi,
        ImageApi, 
        TagsApi, 
        CollectionsApi, 
        ModApi,
        ModerationApi,
    )
}