use criterion::{criterion_group, criterion_main, Criterion};

use aoc2020::day01;
use aoc2020::day02;
use aoc2020::day03;
use aoc2020::day04;
use aoc2020::day05;
use aoc2020::day06;
use aoc2020::day07;
use aoc2020::day08;
use aoc2020::day09;
use aoc2020::day10;
use aoc2020::day11;
use aoc2020::day12;
use aoc2020::day13;
use aoc2020::day14;
use aoc2020::day15;
use aoc2020::day16;
use aoc2020::day17;
use aoc2020::day18;
use aoc2020::day19;
use aoc2020::day21;
use aoc2020::day22;

fn aoc2020_all(c: &mut Criterion) {
    c.bench_function("day01", |b| b.iter(|| day01::run().unwrap()));
    c.bench_function("day02", |b| b.iter(|| day02::run().unwrap()));
    c.bench_function("day03", |b| b.iter(|| day03::run().unwrap()));
    c.bench_function("day04", |b| b.iter(|| day04::run().unwrap()));
    c.bench_function("day05", |b| b.iter(|| day05::run().unwrap()));
    c.bench_function("day06", |b| b.iter(|| day06::run().unwrap()));
    c.bench_function("day07", |b| b.iter(|| day07::run().unwrap()));
    c.bench_function("day08", |b| b.iter(|| day08::run().unwrap()));
    c.bench_function("day09", |b| b.iter(|| day09::run().unwrap()));
    c.bench_function("day10", |b| b.iter(|| day10::run().unwrap()));
    c.bench_function("day11", |b| b.iter(|| day11::run().unwrap()));
    c.bench_function("day12", |b| b.iter(|| day12::run().unwrap()));
    c.bench_function("day13", |b| b.iter(|| day13::run().unwrap()));
    c.bench_function("day14", |b| b.iter(|| day14::run().unwrap()));
    c.bench_function("day15", |b| b.iter(|| day15::run().unwrap()));
    c.bench_function("day16", |b| b.iter(|| day16::run().unwrap()));
    c.bench_function("day17", |b| b.iter(|| day17::run().unwrap()));
    c.bench_function("day18", |b| b.iter(|| day18::run().unwrap()));
    c.bench_function("day19", |b| b.iter(|| day19::run().unwrap()));
    c.bench_function("day21", |b| b.iter(|| day21::run().unwrap()));
    c.bench_function("day22", |b| b.iter(|| day22::run().unwrap()));
}

criterion_group! {
    name = all_days;
    config = Criterion::default().sample_size(200);
    targets = aoc2020_all
}
criterion_main!(all_days);
