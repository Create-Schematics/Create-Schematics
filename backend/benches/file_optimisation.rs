use criterion::{black_box, criterion_group, criterion_main, Criterion};
use tracing::info;

pub fn airship(c: &mut Criterion) {
    // https://createmod.com/schematics/create-airship - 82,665 bytes
    bench("benches/test_data/airship.nbt", "airship", c);
}

pub fn ponder(c: &mut Criterion) {
    //https://github.com/Creators-of-Create/Create/blob/mc1.18/dev/src/main/resources/assets/create/ponder/train_track/chunks.nbt - 1,516 bytes
    bench("benches/test_data/chunks.nbt", "ponder", c);
}

#[cfg(feature="compression")]
fn bench(path: &str, name: &str, c: &mut Criterion) {
    use backend::storage::compression::compress;

    let _ = tracing_subscriber::fmt::try_init();

    let file = std::path::Path::new(path);
    let contents = std::fs::read(file).unwrap();
    let mut smallest = contents.len();

    c.bench_function(&*format!("\"optimise {}\"", name), |b| b.iter(|| {
        let o = compress(black_box(&contents)).unwrap_or_else(|_| contents.clone());
        smallest = smallest.min(o.len())
    }));

    let original_length = contents.len();
    let saved = contents.len() - smallest;    

    info!(%name, %original_length, %smallest, %saved);
}

criterion_group!(benches, ponder, airship);

criterion_main!(benches);