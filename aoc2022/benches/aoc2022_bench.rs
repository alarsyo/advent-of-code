use criterion::{criterion_group, criterion_main, Criterion};

use aoc2022::day01;
use aoc2022::day02;
use aoc2022::day03;

fn aoc2022_all(c: &mut Criterion) {
    c.bench_function("day01", |b| b.iter(|| day01::run().unwrap()));
    c.bench_function("day02", |b| b.iter(|| day02::run().unwrap()));
    c.bench_function("day03", |b| b.iter(|| day03::run().unwrap()));
}

criterion_group! {
    name = all_days;
    config = Criterion::default().sample_size(200);
    targets = aoc2022_all
}
criterion_main!(all_days);
