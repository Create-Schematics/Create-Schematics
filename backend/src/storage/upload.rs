use std::path::PathBuf;
use image::DynamicImage;

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

pub async fn save_schematic_files(
    dir: &TempDir,
    files: Vec<FileUpload>,
    images: Vec<FileUpload>
) -> Result<(), ApiError> {
    let schematic_dir = dir.path().join(super::SCHEMATIC_PATH);
    save_schematics(schematic_dir, files).await?;

    let image_dir = dir.path().join(super::IMAGE_PATH);
    save_images(image_dir, images).await?;

    Ok(())
}   

async fn save_images(location: PathBuf, images: Vec<FileUpload>) -> Result<(), ApiError> {
    let mut files: Vec<String> = vec![];

    std::fs::create_dir(&location).map_err(anyhow::Error::new)?;

    for image in images {
        if files.contains(&image.file_name) {
            // Prevent duplicate requests
            return Err(ApiError::BadRequest);
        }

        let path = location.join(&image.file_name).with_extension("webp");
        files.push(image.file_name);

        let img = image::load_from_memory(&image.contents)
            .map_err(|_| ApiError::BadRequest)?;
        
        // The Webp Encoder doesnt support all image colour formats so standardize to rgb8
        let img = DynamicImage::ImageRgb8(img.into_rgb8());

        let encoder = WebpEncoder::from_image(&img).map_err(|_| ApiError::BadRequest)?;
        let webp = encoder.encode(90f32);

        // Webp memory derefrences to an arbritrarily lengthed byte array and thus can't be safely sent
        // across threads, and therefor can't be directly awaited, if you know a way around this please
        // contact us
        std::fs::write(&path, &*webp).map_err(anyhow::Error::new)?;
    }
    
    Ok(())
}

async fn save_schematics(location: PathBuf, files: Vec<FileUpload>) -> Result<(), ApiError> {
    let mut used_names: Vec<String> = vec![];

    tokio::fs::create_dir(&location)
        .await
        .map_err(anyhow::Error::new)?;
        
    for file in files {
        if !is_nbt(&file) {
            return Err(ApiError::BadRequest)
        }

        if used_names.contains(&file.file_name) {
            // Prevent duplicate files being uploaded
            return Err(ApiError::BadRequest);
        }

        let path = location.join(&file.file_name);
        used_names.push(file.file_name);

        let contents = &file.contents;
        
        #[cfg(feature="compression")]
        let contents = compression::optimise_file_contents(contents)?;

        tokio::fs::write(path, &contents).await.map_err(anyhow::Error::new)?;
    }

    Ok(())
}

fn is_nbt(file: &FileUpload) -> bool {
    if file.file_name().ends_with(".nbt") {
        return true;
    }

    if file.contents.len() < 2 {
        return false;
    }

    let mut magic = [0; 2];
    magic.copy_from_slice(&file.contents[..2]);
    
    magic == GZIP_SIGNATURE
}
