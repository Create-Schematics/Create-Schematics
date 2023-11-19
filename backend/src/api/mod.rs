use axum::Router;
use utoipa::OpenApi;

use crate::cli::server::ApiContext;

pub mod v1;

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Create Schematics REST API",
        version = "0.0.1",
        license(name = "MIT", url = "https://github.com/Rabbitminers/Create-Schematics/blob/master/LICENSE")
    ),
    paths(
        v1::schematics::search_schematics,
        v1::schematics::upload_schematic,
        v1::schematics::get_schematic_by_id,
        v1::schematics::update_schematic_by_id,
        v1::schematics::delete_schematic_by_id,

        v1::users::current_user,
        v1::users::signup,
        v1::users::login,
        v1::users::logout,
    ),
    components(schemas(
        crate::models::user::User,
        
        v1::schematics::SearchQuery,
        v1::schematics::UpdateSchematic,
        v1::schematics::SchematicBuilder,
        
        crate::models::schematic::Schematic,
 
        v1::users::Login,
        v1::users::Signup,
    ))
)]
pub struct ApiDoc;

pub fn configure() -> Router<ApiContext> {
    Router::new()
        .nest("/v1", v1::configure())
}