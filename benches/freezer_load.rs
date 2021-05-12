use criterion::{criterion_group, criterion_main, Criterion};
use std::path::PathBuf;
use xtra::freezer::BlockPart;

const HEADERS: u64 = 999;

pub fn c_load_headers(c: &mut Criterion) {
    dotenv::from_filename("bench.env").expect("Environment file bench.env not found");
    let ancient_folder = PathBuf::from(
        std::env::var("FREEZER_LOAD_HEADERS")
            .expect("Environment variable FREEZER_LOAD_HEADERS not found"),
    );
    c.bench_function(&format!("freezer_load_headers{}", HEADERS), |bencher| {
        bencher.iter(|| {
            BlockPart::Headers
                .load(ancient_folder.as_path(), 0, HEADERS)
                .unwrap()
        })
    });
}

criterion_group! {
name = load_headers;
config = Criterion::default().sample_size(50);
targets = c_load_headers
}
criterion_main!(load_headers);
