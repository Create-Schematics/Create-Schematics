use std::path::PathBuf;

use uuid::Uuid;

pub mod upload;

#[cfg(feature="compression")]
pub mod compression;

pub const UPLOAD_PATH: &'static str = "static/upload/schematics";
pub const SCHEMATIC_PATH: &'static str = "schematics";
pub const IMAGE_PATH: &'static str = "images";

pub fn schematic_image_path(schematic_id: &Uuid) -> PathBuf {
    schematic_upload_path(schematic_id).join(IMAGE_PATH)
}

pub fn schematic_file_path(schematic_id: &Uuid) -> PathBuf {
    schematic_upload_path(schematic_id).join(SCHEMATIC_PATH)
}

pub fn schematic_upload_path(schematic_id: &Uuid) -> PathBuf {
    PathBuf::from(UPLOAD_PATH).join(schematic_id.to_string())
}