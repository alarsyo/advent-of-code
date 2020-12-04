use criterion::{criterion_group, criterion_main, Criterion};

use aoc2020::day01;
use aoc2020::day02;
use aoc2020::day03;
use aoc2020::day04;

fn aoc2020_all(c: &mut Criterion) {
    c.bench_function("day01", |b| b.iter(|| day01::run().unwrap()));
    c.bench_function("day02", |b| b.iter(|| day02::run().unwrap()));
    c.bench_function("day03", |b| b.iter(|| day03::run().unwrap()));
    c.bench_function("day04", |b| b.iter(|| day04::run().unwrap()));
}

criterion_group! {
    name = all_days;
    config = Criterion::default().sample_size(30);
    targets = aoc2020_all
}
criterion_main!(all_days);
