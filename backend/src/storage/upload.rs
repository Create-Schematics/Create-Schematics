use std::path::PathBuf;

use tempfile::{Builder, TempDir};
use uuid::Uuid;
use crate::{error::ApiError, middleware::files::FileUpload};

pub fn build_upload_directory(
    schematic_id: &Uuid
) -> Result<TempDir, anyhow::Error> {
    Builder::new()
        .prefix(&schematic_id.to_string())
        .rand_bytes(4)
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
        let file_name = image.metadata.file_name.ok_or(ApiError::BadRequest)?;
        let sanitized = sanitize_filename::sanitize(&file_name);

        if files.contains(&sanitized) {
            // Prevent duplicate requests
            return Err(ApiError::BadRequest);
        }

        let image_buffer = image::load_from_memory(&image.contents)
            .map_err(|_| ApiError::BadRequest)?;

        let path = location.join(&sanitized);
        files.push(sanitized);

        image_buffer.save(path).map_err(anyhow::Error::new)?;
    }

    Ok(files)
}

fn save_schematics(location: PathBuf, files: Vec<FileUpload>) -> Result<Vec<String>, ApiError> {
    let mut output: Vec<String> = vec![];

    std::fs::create_dir(&location).map_err(anyhow::Error::new)?;
    
    for file in files {
        let file_name = file.metadata.file_name.ok_or(ApiError::BadRequest)?;
        
        if !file_name.ends_with(".nbt") {
            return Err(ApiError::BadRequest)
        }

        let sanitized = sanitize_filename::sanitize(&file_name);

        if output.contains(&sanitized) {
            // Prevent duplicate requests
            return Err(ApiError::BadRequest);
        }

        let path = location.join(&sanitized);
        output.push(sanitized);
        
        std::fs::write(path, file.contents).map_err(anyhow::Error::new)?;
    }

    Ok(output)
}