use std::fs::File;
use std::io::Write;

use clap::Args;
use utoipa::{OpenApi, Modify};

use crate::authentication::session::Session;

use super::auth;
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
        license(name = "MIT", url = "https://github.com/Create-Schematics/Create-Schematics/blob/master/LICENSE")
    ),
    paths(
        auth::oauth_authorization,
        auth::oauth_callback,

        v1::users::current_user,
        v1::users::update_current_user,
        v1::users::remove_current_user,

        v1::profile::get_uploaded_schematics,

        v1::schematics::search_schematics,
        v1::schematics::upload_schematic,
        v1::schematics::get_schematic_by_id,
        v1::schematics::update_schematic_by_id,
        v1::schematics::delete_schematic_by_id,

        v1::images::get_images_from_schematic,
        v1::images::upload_image_to_schematic,
        v1::images::remove_image_from_schematic,

        v1::files::get_files_from_schematic,
        v1::files::upload_file_to_schematic,
        v1::files::remove_file_from_schematic,
        
        v1::comments::get_comments_by_schematic,
        v1::comments::post_comment,
        v1::comments::get_comment_by_id,
        v1::comments::update_comment_by_id,
        v1::comments::delete_comment_by_id,

        v1::collections::collections_containing_schematic,
        v1::collections::get_users_collections,
        v1::collections::get_current_users_collections,
        v1::collections::create_new_collection,
        v1::collections::get_collection_by_id,
        v1::collections::remove_collection_by_id,
        v1::collections::fetch_collection_entries,
        v1::collections::add_schematic_to_collection,
        v1::collections::remove_schematic_from_collection,

        v1::tags::get_schematic_tags,
        v1::tags::tag_schematic_by_id,
        v1::tags::get_valid_tags,
        v1::tags::untag_schematic_by_id,

        v1::likes::like_schematic,
        v1::likes::remove_like_from_schematic
    ),
    modifiers(
        &AuthenticationModifier
    ),
    components(schemas(
        auth::AuthRequest,
        auth::OauthProviders,

        crate::models::user::User,
        
        v1::users::UpdateUser,
        
        crate::models::schematic::Schematic,
        
        v1::schematics::FullSchematic,
        v1::schematics::SortBy,
        v1::schematics::SearchQuery,
        v1::schematics::UpdateSchematic,
        v1::schematics::SchematicBuilder,
        
        v1::images::Images,
        v1::images::UploadImage,
        v1::images::DeleteImage,

        v1::files::Files,
        v1::files::UploadFile,
        v1::files::DeleteFile,
        
        crate::models::comment::Comment,

        v1::comments::PaginationQuery,
        v1::comments::FullComment,
        v1::comments::CommentBuilder,
        v1::comments::UpdateComment,

        v1::collections::Collection,
        v1::collections::UserCollection,
        v1::collections::FullCollection,
        v1::collections::UpdateCollection,
        v1::collections::CollectionBuilder,
        v1::collections::CollectionEntry,

        v1::tags::Tags,
        v1::tags::FullTag,

        v1::likes::LikeQuery,
        v1::likes::LikeAction
    ))
)]
pub struct ApiDoc;

#[derive(Args, Debug)]
pub struct OpenApiSchemaCommandArguements {
    #[arg(help = "Weather to output a yaml schema")]
    #[arg(short = 'y', long = "yaml")]
    #[arg(default_value = "true")]
    pub yaml: bool, 

    #[arg(help = "Weather to output a json schema")]
    #[arg(short = 'j', long = "json")]
    #[arg(default_value = "true")]
    pub json: bool, 
}

pub fn save_schema(
    OpenApiSchemaCommandArguements {
        yaml,
        json,
        ..
    }: OpenApiSchemaCommandArguements
) -> Result<(), anyhow::Error>{
    let openapi = ApiDoc::openapi();

    if json {
        let mut output = File::create("openapi.json")?;
        let schema = openapi.to_pretty_json()?;

        output.write_all(schema.as_bytes())?;
    }

    if yaml {
        let mut output = File::create("openapi.yaml")?;
        let schema = openapi.to_yaml()?;

        output.write_all(schema.as_bytes())?;
    }

    Ok(())
}