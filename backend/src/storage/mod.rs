pub mod upload;

#[cfg(feature="compression")]
pub mod compression;

pub const UPLOAD_PATH: &'static str = "static/upload/schematics";
pub const SCHEMATIC_PATH: &'static str = "schematics";
pub const IMAGE_PATH: &'static str = "images";