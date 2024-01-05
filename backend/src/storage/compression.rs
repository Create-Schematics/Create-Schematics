use libdeflater::{CompressionLvl, Compressor};
use zune_inflate::DeflateDecoder as GzDecoder;

pub fn optimise_file_contents(input: &Vec<u8>) -> Result<Vec<u8>, anyhow::Error> {
    let contents = decompress(&input)?;

    compress(&contents)
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
