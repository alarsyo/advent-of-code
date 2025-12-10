use criterion::{Criterion, criterion_group, criterion_main};

use aoc2025::day01;
use aoc2025::day02;
use aoc2025::day03;
use aoc2025::day04;
use aoc2025::day05;
use aoc2025::day06;
use aoc2025::day07;

fn aoc2025_all(c: &mut Criterion) {
    c.bench_function("day01", |b| b.iter(|| day01::run().unwrap()));
    c.bench_function("day02", |b| b.iter(|| day02::run().unwrap()));
    c.bench_function("day03", |b| b.iter(|| day03::run().unwrap()));
    c.bench_function("day04", |b| b.iter(|| day04::run().unwrap()));
    c.bench_function("day05", |b| b.iter(|| day05::run().unwrap()));
    c.bench_function("day06", |b| b.iter(|| day06::run().unwrap()));
    c.bench_function("day07", |b| b.iter(|| day07::run().unwrap()));
}

criterion_group! {
    name = all_days;
    config = Criterion::default().sample_size(200);
    targets = aoc2025_all
}
criterion_main!(all_days);
