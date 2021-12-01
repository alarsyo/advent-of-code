use criterion::{criterion_group, criterion_main, Criterion};

use aoc2021::day01;

fn aoc2021_all(c: &mut Criterion) {
    c.bench_function("day01", |b| b.iter(|| day01::run().unwrap()));
}

criterion_group! {
    name = all_days;
    config = Criterion::default().sample_size(200);
    targets = aoc2021_all
}
criterion_main!(all_days);
