use criterion::{criterion_group, criterion_main, Criterion};

use aoc2022::day01;

fn aoc2022_all(c: &mut Criterion) {
    c.bench_function("day01", |b| b.iter(|| day01::run().unwrap()));
}

criterion_group! {
    name = all_days;
    config = Criterion::default().sample_size(200);
    targets = aoc2022_all
}
criterion_main!(all_days);
