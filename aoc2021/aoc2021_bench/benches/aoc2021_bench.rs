use criterion::{criterion_group, criterion_main, Criterion};

use aoc2021::day00;

fn aoc2021_all(c: &mut Criterion) {
    c.bench_function("day00", |b| b.iter(|| day00::run().unwrap()));
}

criterion_group! {
    name = all_days;
    config = Criterion::default().sample_size(200);
    targets = aoc2021_all
}
criterion_main!(all_days);
