use std::io::{self, Error};
use std::path::PathBuf;
use image::DynamicImage;
use libdeflater::{CompressionLvl, Compressor};

use webp::Encoder as WebpEncoder;

use tempfile::{Builder, TempDir};
use uuid::Uuid;
use crate::{error::ApiError, middleware::files::FileUpload};

use zune_inflate::DeflateDecoder as GzDecoder;

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
        let webp = encoder.encode(100f32);

        std::fs::write(&path, &*webp).map_err(anyhow::Error::new)?;
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

pub fn optimise_file_contents(input: Vec<u8>) -> Vec<u8> {
    let contents = match decompress(input.clone()) {
        Ok(c) => c,
        Err(_) => return input
    };

    compress(contents).unwrap_or_else(|_| input)
}

fn decompress(stuff: Vec<u8>) -> io::Result<Vec<u8>> {
    let mut decoder = GzDecoder::new(&stuff[..]);

    match decoder.decode_gzip() {
        Ok(result) => Ok(result),
        Err(_) => Err(Error::new(io::ErrorKind::InvalidData, "Invalid gzip data"))
    }
}

fn compress(data: Vec<u8>) -> io::Result<Vec<u8>> {
    let mut compressor = Compressor::new(CompressionLvl::new(12).unwrap());
    let capacity = compressor.gzip_compress_bound(data.len());
    let mut dest = vec![0; capacity];
    match compressor.gzip_compress(&*data, &mut dest) {
        Ok(len) => {
            dest.truncate(len);
            Ok(dest)
        }
        Err(e) => Err(Error::new(io::ErrorKind::InvalidData, e))
    }
}
