use poem::Route;
use poem_openapi::{OpenApiService, LicenseObject, ContactObject};

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

pub (in crate::api) fn configure() -> Route {
    let apis = (
        CommentsApi, 
        SchematicsApi, 
        LikesApi, 
        UsersApi, 
        TagsApi, 
        CollectionsApi, 
        ImageApi, 
        FileApi
    );
    
    let license = LicenseObject::new("MIT")
        .url("https://github.com/Create-Schematics/Create-Schematics/blob/master/LICENSE");

    let contact = ContactObject::new()
        .name("Create-Schematics")
        .url("https://github.com/Create-Schematics");

    let api_service = OpenApiService::new(apis, "Create Schematics REST API", "0.1")
        .server("/api/v1")
        .license(license)
        .contact(contact)
        .external_document("https://github.com/Create-Schematics/Create-Schematics");

    let spec_json = api_service.spec();
    let spec_yaml = api_service.spec_yaml();

    Route::new()
        .nest("/", api_service)
        .at("/openapi.json", spec_json)
        .at("/openapi.yaml", spec_yaml)
}