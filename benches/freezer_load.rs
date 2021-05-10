use criterion::{criterion_group, criterion_main, Criterion};
use std::path::PathBuf;
use xtra::freezer::BlockPart;

const BLOCKS: u64 = 100_000;

pub fn c_load_bodies(c: &mut Criterion) {
    dotenv::from_filename("bench.env").expect("Environment file bench.env not found");
    let ancient_folder = PathBuf::from(
        std::env::var("FREEZER_LOAD_BODIES")
            .expect("Environment variable FREEZER_LOAD_BODIES not found"),
    );
    c.bench_function(&format!("freezer_load_bodies_{}", BLOCKS), |bencher| {
        bencher.iter(|| {
            BlockPart::Bodies
                .load(ancient_folder.as_path(), 0, BLOCKS)
                .unwrap()
        })
    });
}

criterion_group! {
name = load_bodies;
config = Criterion::default();
targets = c_load_bodies
}
criterion_main!(load_bodies);
