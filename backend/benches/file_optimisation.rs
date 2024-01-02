use criterion::{black_box, criterion_group, criterion_main, Criterion};
use backend::storage::upload::optimise_file_contents;
use libdeflater::*;

pub fn airship_very_slow(c: &mut Criterion) {
    // https://createmod.com/schematics/create-airship - 82,665 bytes
    libdeflater("benches/test_data/airship.nbt", "airship", c);
    // zopfli("benches/test_data/airship.nbt", "airship", c);
}

pub fn ponder(c: &mut Criterion) {
    //https://github.com/Creators-of-Create/Create/blob/mc1.18/dev/src/main/resources/assets/create/ponder/train_track/chunks.nbt - 1,516 bytes
    libdeflater("benches/test_data/chunks.nbt", "ponder", c);
    zopfli("benches/test_data/chunks.nbt", "ponder", c);
}

fn zopfli(path: &str, name: &str, c: &mut Criterion) {
    let file = std::path::Path::new(path);
    let contents = std::fs::read(file).unwrap();
    let mut smallest = contents.len();
    let name = format!("optimise {} with zopfli", name);
    c.bench_function(&*name, |b| b.iter(|| {
        let o = optimise_file_contents(black_box(contents.clone()));
        smallest = smallest.min(o.len())
    }));

    println!("zopfli: saved {}", contents.len() - smallest);
}

fn libdeflater(path: &str, name: &str, c: &mut Criterion) {
    let file = std::path::Path::new(path);
    let contents = std::fs::read(file).unwrap();
    let mut smallest = contents.len();
    let name = format!("optimise {} with libdeflater", name);
    c.bench_function(&*name, |b| b.iter(|| {

        let uncompressed = match inflate(&contents) {
            Ok(v) => v,
            Err(e) => panic!("Failed to decompress: {:?}", e)
        };
        let o = deflate(&uncompressed, 9);
        smallest = smallest.min(o.len())
    }));

    println!("libdeflater: saved {}", contents.len() - smallest);
}

fn deflate(data: &Vec<u8>, level: u8) -> Vec<u8> {
    let mut compressor = Compressor::new(CompressionLvl::new(level.into()).unwrap());
    let capacity = compressor.gzip_compress_bound(data.len());
    let mut dest = vec![0; capacity];
    let len = compressor
        .gzip_compress(data, &mut dest)
        .unwrap();
    dest.truncate(len);
    dest
}

fn inflate(data: &Vec<u8>) -> Result<Vec<u8>, DecompressionError> {
    let mut decompressor = Decompressor::new();
    let mut dest = vec![0; data.len() * 10];
    loop {
        match decompressor.gzip_decompress(data, &mut dest) {
            Ok(len) => {
                dest.truncate(len);
                return Ok(dest);
            }
            Err(DecompressionError::InsufficientSpace) => {
                dest.resize(dest.len() * 2, 0);
            }
            Err(e) => return Err(e),
        }
    }
}

criterion_group!(benches, ponder, airship_very_slow);
criterion_main!(benches);