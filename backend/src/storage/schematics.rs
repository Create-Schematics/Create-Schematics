use std::borrow::Cow;
use std::collections::HashSet;
use zune_inflate::DeflateDecoder as GzDecoder;

use crate::error::ApiError;
use crate::response::ApiResult;

#[derive(Deserialize, Debug)]
pub struct Schematic<'a> {
    pub palette: Vec<PaletteEntry<'a>>
}

#[derive(Deserialize, Debug)]
pub struct PaletteEntry<'a> {
    #[serde(rename="Name")]
    pub name: Cow<'a, str>
}

pub fn extract_modlist(contents: &Vec<u8>) -> Result<HashSet<String>, ApiError> {
    let decompressed = decompress(&contents)?;
    
    let schematic = fastnbt::from_bytes::<Schematic>(&decompressed)
        .map_err(|_| ApiError::BadRequest)?;

    let mod_list: HashSet<String> = schematic.palette
        .iter()
        .filter_map(|e| e.name.split(":").next())
        .map(|mod_id| mod_id.to_string())
        .collect();

    Ok(mod_list)
}

pub fn decompress(data: &Vec<u8>) -> ApiResult<Vec<u8>> {
    let mut decoder = GzDecoder::new(&data[..]);
    let decoded = decoder.decode_gzip().map_err(|_| ApiError::BadRequest)?;

    Ok(decoded)
}