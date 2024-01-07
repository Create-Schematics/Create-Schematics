use std::path::PathBuf;
use image::DynamicImage;

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use webp::Encoder as WebpEncoder;

use tempfile::{Builder, TempDir};
use uuid::Uuid;
use crate::{error::ApiError, middleware::files::FileUpload};

#[cfg(feature="compression")]
use crate::storage::compression;

// https://gist.github.com/leommoore/f9e57ba2aa4bf197ebc5#archive-files
const GZIP_SIGNATURE: [u8; 2] = [0x1f, 0x8b];

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
    images.par_iter()
        .map(|image| -> Result<String, ApiError> {
            // Since this needs to be returned I don't think the clone is avoidable here,
            // todo: look into alternate implementations so this isn't needed
            let file_name = image.file_name.clone().ok_or(ApiError::BadRequest)?;
            save_image(&path, &file_name, &image.contents)?;

            Ok(file_name)
        })
        .collect::<Result<Vec<String>, ApiError>>()
}

pub fn save_image(location: &PathBuf, file_name: &str, contents: &Vec<u8>) -> Result<(), ApiError> {
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

pub async fn save_schematics(location: &TempDir, files: Vec<FileUpload>) -> Result<Vec<String>, ApiError> {
    let path = location.path().join(super::SCHEMATIC_PATH);
    tokio::fs::create_dir(&path).await.map_err(anyhow::Error::new)?;
    
    // Unlike image uploads processing nbt files is much cheaper, although if the feature is
    // enabled they will be compressed so we still upload them in parralel
    files.par_iter()
        .map(|file| -> Result<String, ApiError> {
            // Since this needs to be returned I don't think the clone is avoidable here,
            // todo: look into alternate implementations so this isn't needed
            let file_name = file.file_name.clone().ok_or(ApiError::BadRequest)?;
            save_schematic(&path, &file_name, &file.contents)?;

            Ok(file_name)
        })
        .collect::<Result<Vec<String>, ApiError>>()
}

pub fn save_schematic(location: &PathBuf, file_name: &str, contents: &Vec<u8>) -> Result<(), ApiError> {
    if !is_nbt(&file_name, &contents) {
        return Err(ApiError::BadRequest)
    }
    
    let path = location.join(&file_name);

    if path.exists() {
        return Err(ApiError::BadRequest);
    }

    #[cfg(feature="compression")]
    let contents = compression::optimise_file_contents(contents)?;

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
