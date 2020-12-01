use criterion::{criterion_group, criterion_main, Criterion};

use aoc2020::day01;

fn aoc2020_all(c: &mut Criterion) {
    c.bench_function("day01", |b| b.iter(|| day01::run().unwrap()));
}

criterion_group! {
    name = all_days;
    config = Criterion::default().sample_size(30);
    targets = aoc2020_all
}
criterion_main!(all_days);
