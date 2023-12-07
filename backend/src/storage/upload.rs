use std::path::PathBuf;

use axum::body::Bytes;
use axum_typed_multipart::FieldData;
use tempfile::Builder;
use uuid::Uuid;

use crate::error::ApiError;

pub async fn save_schematic_files(
    schematic_id: Uuid,
    files: Vec<FieldData<Bytes>>,
    images: Vec<FieldData<Bytes>>
) -> Result<(Vec<String>, Vec<String>), ApiError> {
    let dir = Builder::new()
        .prefix(&schematic_id.to_string())
        .rand_bytes(4)
        .tempdir_in(super::UPLOAD_PATH)
        .map_err(anyhow::Error::new)?;

    let image_dir = dir.path().join(super::IMAGE_PATH);
    let images = save_images(image_dir, images)?;

    let schematic_dir = dir.path().join(super::SCHEMATIC_PATH);
    let schematics = save_schematics(schematic_dir, files)?;

    let _persist = dir.into_path();

    Ok((schematics, images))
}   

fn save_images(location: PathBuf, images: Vec<FieldData<Bytes>>) -> Result<Vec<String>, ApiError> {
    let mut files: Vec<String> = vec![];

    for image in images {
        let file_name = image.metadata.file_name.ok_or(ApiError::BadRequest)?;
        let sanitized = sanitize_filename::sanitize(&file_name);

        let image_buffer = image::load_from_memory(&image.contents)
            .map_err(|_| ApiError::BadRequest)?;

        let mut path = location.join(&sanitized);
        path.set_extension("webp");

        files.push(sanitized);

        image_buffer.save(path).map_err(anyhow::Error::new)?;
    }

    Ok(files)
}

fn save_schematics(location: PathBuf, files: Vec<FieldData<Bytes>>) -> Result<Vec<String>, ApiError> {
    let mut output: Vec<String> = vec![];
    
    for file in files {
        let file_name = file.metadata.file_name.ok_or(ApiError::BadRequest)?;
        
        if !file_name.ends_with(".nbt") {
            return Err(ApiError::BadRequest)
        }

        let sanitized = sanitize_filename::sanitize(&file_name);
        let path = location.join(&sanitized);
        
        output.push(sanitized);
        
        std::fs::write(path, file.contents).map_err(anyhow::Error::new)?;
    }

    Ok(output)
}