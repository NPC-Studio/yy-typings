use criterion::{black_box, criterion_group, criterion_main, Criterion};
use yy_typings::*;

pub fn criterion_benchmark(c: &mut Criterion) {
    let input = include_str!("../data/formatting/floater.yy");
    let value: Sprite =
        serde_json::from_str(&TrailingCommaUtility::clear_trailing_comma_once(input)).unwrap();

    c.bench_function("floater", |b| {
        b.iter(|| {
            black_box(serialize_file(&value));
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
