use libdeflater::{CompressionLvl, Compressor};

pub fn compress(data: &Vec<u8>) -> Result<Vec<u8>, anyhow::Error> {
    let mut compressor = Compressor::new(CompressionLvl::best());
    let capacity = compressor.gzip_compress_bound(data.len());
    let mut dest = vec![0; capacity];

    let len = compressor.gzip_compress(&*data, &mut dest).map_err(anyhow::Error::new)?;

    dest.truncate(len);
    Ok(dest)
}
