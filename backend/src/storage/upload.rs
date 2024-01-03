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

        let optimized_contents = optimise_file_contents(&file.contents)
            .unwrap_or_else(|| file.contents);
        std::fs::write(path, optimized_contents).map_err(anyhow::Error::new)?;
    }

    Ok(output)
}

fn is_nbt(file: &FileUpload) -> bool {
    if file.file_name.as_ref()
        .map(|name| name.ends_with(".nbt"))
        .unwrap_or(false) {
        return true;
    }

    let gzip_magic = [0x1f, 0x8b];
    if file.contents.len() > 2 {
        let mut magic = [0; 2];
        magic.copy_from_slice(&file.contents[..2]);
        if magic == gzip_magic {
            return true;
        }
    }

    false
}

pub fn optimise_file_contents(input: &Vec<u8>) -> Option<Vec<u8>> {
    let contents = match decompress(&input) {
        Ok(c) => c,
        Err(_) => return None
    };

    compress(&contents).ok()
}

fn decompress(data: &Vec<u8>) -> Result<Vec<u8>, anyhow::Error> {
    let mut decoder = GzDecoder::new(&data[..]);

    match decoder.decode_gzip() {
        Ok(c) => Ok(c),
        Err(e) => {
            Err(anyhow::Error::msg(e.to_string()))
        }
    }
}

fn compress(data: &Vec<u8>) -> Result<Vec<u8>, anyhow::Error> {
    let mut compressor = Compressor::new(CompressionLvl::best());
    let capacity = compressor.gzip_compress_bound(data.len());
    let mut dest = vec![0; capacity];

    let len = compressor.gzip_compress(&*data, &mut dest).map_err(anyhow::Error::new)?;

    dest.truncate(len);
    Ok(dest)
}
