use criterion::{black_box, criterion_group, criterion_main, Criterion};
use backend::storage::upload::optimise_file_contents;

pub fn airship_very_slow(c: &mut Criterion) {
    // https://createmod.com/schematics/create-airship - 82,665 bytes
    bench("benches/test_data/airship.nbt", "airship", c);
}

pub fn ponder(c: &mut Criterion) {
    //https://github.com/Creators-of-Create/Create/blob/mc1.18/dev/src/main/resources/assets/create/ponder/train_track/chunks.nbt - 1,516 bytes
    bench("benches/test_data/chunks.nbt", "ponder", c);
}

fn bench(path: &str, name: &str, c: &mut Criterion) {
    let file = std::path::Path::new(path);
    let contents = std::fs::read(file).unwrap();
    let mut smallest = contents.len();
    let name = format!("optimise {}", name);
    c.bench_function(&*name, |b| b.iter(|| {
        let o = optimise_file_contents(black_box(contents.clone()));
        smallest = smallest.min(o.len())
    }));

    println!("original: {}, optimised: {}", contents.len(), smallest);
}

criterion_group!(benches, ponder/*, airship_very_slow */);
criterion_main!(benches);