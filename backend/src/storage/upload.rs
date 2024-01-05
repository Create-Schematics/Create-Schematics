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
) -> Result<(Vec<String>, Vec<String>), ApiError> {
    let schematic_dir = dir.path().join(super::SCHEMATIC_PATH);
    let schematics = save_schematics(schematic_dir, files)?;

    let image_dir = dir.path().join(super::IMAGE_PATH);
    let images = save_images(image_dir, images)?;

    Ok((schematics, images))
}   

fn save_images(location: PathBuf, images: Vec<FileUpload>) -> Result<Vec<String>, ApiError> {
    let mut files: Vec<String> = vec![];

    std::fs::create_dir(&location).map_err(anyhow::Error::new)?;

    for image in images {
        let file_name = image.file_name.ok_or(ApiError::BadRequest)?;
        let sanitized = sanitize_filename::sanitize(&file_name);

        if files.contains(&sanitized) {
            // Prevent duplicate requests
            return Err(ApiError::BadRequest);
        }

        let path = location.join(&sanitized).with_extension("webp");
        files.push(sanitized);

        let img = image::load_from_memory(&image.contents)
            .map_err(|_| ApiError::BadRequest)?;

        let img = DynamicImage::ImageRgb8(img.into_rgb8());

        let encoder = WebpEncoder::from_image(&img).unwrap();
        let webp = encoder.encode(90f32);

        std::fs::write(&path, &*webp).map_err(anyhow::Error::new)?;
    }

    Ok(files)
}

fn save_schematics(location: PathBuf, files: Vec<FileUpload>) -> Result<Vec<String>, ApiError> {
    let mut output: Vec<String> = vec![];

    std::fs::create_dir(&location).map_err(anyhow::Error::new)?;
    
    for file in files {
        let file_name = file.file_name.as_ref().ok_or(ApiError::BadRequest)?;
        
        if !is_nbt(&file) {
            return Err(ApiError::BadRequest)
        }

        let sanitized = sanitize_filename::sanitize(&file_name);

        if output.contains(&sanitized) {
            // Prevent duplicate requests
            return Err(ApiError::BadRequest);
        }

        let path = location.join(&sanitized);
        output.push(sanitized);

        let contents = &file.contents;
        
        #[cfg(feature="compression")]
        let contents = compression::optimise_file_contents(contents)?;

        std::fs::write(path, contents).map_err(anyhow::Error::new)?;
    }

    Ok(output)
}

fn is_nbt(file: &FileUpload) -> bool {
    if file.file_name.as_ref()
        .map(|name| name.ends_with(".nbt"))
        .unwrap_or(false) 
    {
        return true;
    }

    if file.contents.len() < 2 {
        return false;
    }

    let mut magic = [0; 2];
    magic.copy_from_slice(&file.contents[..2]);
    
    magic == GZIP_SIGNATURE
}
