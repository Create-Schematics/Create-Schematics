use poem::Route;
use poem_openapi::{OpenApiService, LicenseObject, ContactObject, OpenApi};

use self::likes::LikesApi;
use self::files::FileApi;
use self::images::ImageApi;
use self::collections::CollectionsApi;
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

pub (in crate::api) fn configure() -> impl OpenApi {
    (
        CommentsApi, 
        SchematicsApi, 
        LikesApi, 
        UsersApi, 
        TagsApi, 
        CollectionsApi, 
        ImageApi, 
        FileApi
    )
}