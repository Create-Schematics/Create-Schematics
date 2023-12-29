use std::io::{self, Cursor, Read};
use std::num::NonZeroU64;
use std::path::PathBuf;

use tempfile::{Builder, TempDir};
use uuid::Uuid;
use crate::{error::ApiError, middleware::files::FileUpload};

use flate2::write::GzDecoder;

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

        let path = location.join(&sanitized);
        files.push(sanitized);

        image::load_from_memory(&image.contents)
            .map_err(|_| ApiError::BadRequest)?
            .save(path)
            .map_err(anyhow::Error::new)?;
    }

    Ok(files)
}

fn save_schematics(location: PathBuf, files: Vec<FileUpload>) -> Result<Vec<String>, ApiError> {
    let mut output: Vec<String> = vec![];

    std::fs::create_dir(&location).map_err(anyhow::Error::new)?;
    
    for file in files {
        let file_name = file.file_name.ok_or(ApiError::BadRequest)?;
        
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

        let optimized_contents = optimise_file_contents(file.contents);
        std::fs::write(path, optimized_contents).map_err(anyhow::Error::new)?;
    }

    Ok(output)
}

fn optimise_file_contents(input: Vec<u8>) -> Vec<u8> {
    let contents = match decompress(input.clone()) {
        Ok(c) => c,
        Err(_) => return input
    };

    let iter = if contents.len() > 20_000 { 100 } else { 500 };

    compress(contents, iter).unwrap_or_else(|_| input)
}

fn decompress(stuff: Vec<u8>) -> io::Result<Vec<u8>> {
    let mut decoder = GzDecoder::new(Cursor::new(stuff.clone()));
    let mut result = Vec::new();

    match decoder.read_to_end(&mut result) {
        Ok(_) => Ok(result),
        Err(e) => Err(e)
    }
}

fn compress(stuff: Vec<u8>, iter: u64) -> io::Result<Vec<u8>> {
    let options = zopfli::Options {
        iteration_count: NonZeroU64::new(iter).unwrap(),
        ..Default::default()
    };

    let mut output = Vec::with_capacity(stuff.len());
    match zopfli::compress(options, zopfli::Format::Zlib, &stuff[..], &mut output) {
        Ok(_) => {
            output.shrink_to_fit();
            Ok(output)
        },
        Err(e) => Err(e)
    }
}