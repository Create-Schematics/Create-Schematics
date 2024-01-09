use poem_openapi::OpenApi;

use self::likes::LikesApi;
use self::files::FileApi;
use self::images::ImageApi;
use self::collections::CollectionsApi;
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

pub fn configure() -> impl OpenApi {
    (
        CommentsApi, 
        SchematicsApi, 
        LikesApi, 
        UsersApi, 
        TagsApi, 
        CollectionsApi, 
        ModApi,
        ImageApi, 
        FileApi
    )
}