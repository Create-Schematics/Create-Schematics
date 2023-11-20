use utoipa::{OpenApi, Modify};

use crate::authentication::session::Session;

use super::v1;

struct AuthenticationModifier;

impl Modify for AuthenticationModifier {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme("session_cookie",  Session::security_scheme())
        }
    }
}

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Create Schematics REST API",
        version = "0.0.1",
        license(name = "MIT", url = "https://github.com/Rabbitminers/Create-Schematics/blob/master/LICENSE")
    ),
    paths(
        v1::favorites::get_favorites,
        v1::favorites::favorite_schematic,
        v1::favorites::unfavorite_schematic,

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
    modifiers(
        &AuthenticationModifier
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