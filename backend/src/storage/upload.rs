use std::path::PathBuf;
use std::collections::HashSet;
use image::DynamicImage;

use rayon::iter::{ParallelIterator, IntoParallelIterator, IntoParallelRefIterator};
use webp::Encoder as WebpEncoder;

use tempfile::{Builder, TempDir};
use uuid::Uuid;
use crate::{error::ApiError, middleware::files::FileUpload};

#[cfg(feature="compression")]
use crate::storage::compression;

use super::schematics::{decompress, extract_modlist};

// https://gist.github.com/leommoore/f9e57ba2aa4bf197ebc5#archive-files
const GZIP_SIGNATURE: [u8; 2] = [0x1f, 0x8b];

const MAX_FILE_SIZE: usize = 256 * 1024; // 256kb
const MAX_IMAGE_SIZE: usize = 5 * 1024 * 1024; // 5mb

pub struct SchematicTransfer {
    pub file_name: String,
    pub requirements: HashSet<String>
}

pub fn build_upload_directory(
    schematic_id: &Uuid
) -> Result<TempDir, anyhow::Error> {
    Builder::new()
        .prefix(&schematic_id.to_string())
        .tempdir_in(super::UPLOAD_PATH)
        .map_err(anyhow::Error::new)
}

pub async fn save_images(location: &TempDir, images: Vec<FileUpload>) -> Result<Vec<String>, ApiError> {
    let path = location.path().join(super::IMAGE_PATH);
    tokio::fs::create_dir(&path).await.map_err(anyhow::Error::new)?;
    
    // When uploading multiple images we'll want to parallize processing them since this can
    // be quite slow especially for larger images. In testing within the limits of enforced
    // higher up (10 images up to 5mb) this allows for all images to be processed within the
    // timespan of most costly image signifigantly improving response times 
    images.into_par_iter()
        .map(|image| -> Result<String, ApiError> {
            // We consume the files vector here so we dont need to clone the file 
            // name
            let file_name = image.file_name.ok_or(ApiError::BadRequest)?;
            save_image(&path, &file_name, &image.contents)?;

            Ok(file_name)
        })
        .collect::<Result<Vec<String>, ApiError>>()
}

pub fn save_image(location: &PathBuf, file_name: &str, contents: &Vec<u8>) -> Result<(), ApiError> {
    if contents.len() > MAX_IMAGE_SIZE {
        return Err(ApiError::BadRequest)
    }

    let path = location.join(&file_name).with_extension("webp");

    let img = image::load_from_memory(&contents)
        .map_err(|_| ApiError::BadRequest)?;
    
    // The Webp Encoder doesnt support all image colour formats so standardize to rgb8
    let img = DynamicImage::ImageRgb8(img.into_rgb8());

    let encoder = WebpEncoder::from_image(&img).map_err(|_| ApiError::BadRequest)?;
    let webp = encoder.encode(90f32).to_vec();

    std::fs::write(&path, &webp).map_err(anyhow::Error::new)?;

    Ok(())
}

// todo: replace this, it has numerous issues. First the schematics are decoded twice,
// once when extracting the file list, the again when being uploaded. Second, all files
// will be decoded then checked for their size, format etc. 
pub async fn save_schematics(location: &TempDir, files: Vec<FileUpload>) -> Result<(Vec<String>, HashSet<String>), ApiError> {
    let path = location.path().join(super::SCHEMATIC_PATH);
    tokio::fs::create_dir(&path).await.map_err(anyhow::Error::new)?;

    let mods: HashSet<String> = files.par_iter()
        .filter_map(|file| extract_modlist(&file.contents).ok())
        .flatten()
        .collect();

    // Unlike image uploads processing nbt files is much cheaper, although if the feature is
    // enabled they will be compressed so we still upload them in parralel
    let files = files.into_par_iter()
        .map(|file| -> Result<String, ApiError> {
            // We consume the files vector here so we dont need to clone the file 
            // name
            let file_name = file.file_name.ok_or(ApiError::BadRequest)?;
            save_schematic(&path, &file_name, &file.contents)?;

            Ok(file_name)
        })
        .collect::<Result<Vec<String>, ApiError>>()?;

    Ok((files, mods))
}

pub fn save_schematic(location: &PathBuf, file_name: &str, contents: &Vec<u8>) -> Result<(), ApiError> {
    if contents.len() > MAX_FILE_SIZE || !is_nbt(&file_name, &contents) {
        return Err(ApiError::BadRequest)
    }

    let path = location.join(&file_name);

    if path.exists() {
        return Err(ApiError::BadRequest);
    }

    let contents = decompress(&contents)?;

    #[cfg(feature="compression")]
    let contents = compression::compress(&contents)?;

    std::fs::write(path, &contents).map_err(anyhow::Error::new)?;
    Ok(())
}

fn is_nbt(file_name: &str, contents: &Vec<u8>) -> bool {
    if file_name.ends_with(".nbt") {
        return true;
    }

    if contents.len() < 2 {
        return false;
    }

    let mut magic = [0; 2];
    magic.copy_from_slice(&contents[..2]);
    
    magic == GZIP_SIGNATURE
}
