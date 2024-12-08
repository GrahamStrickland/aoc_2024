use criterion::{criterion_group, criterion_main, Criterion};
pub use aoc_2024::day1::day1;

fn day1_benchmark(c: &mut Criterion) {
    c.bench_function("day1", |b| b.iter(|| day1()));
}

criterion_group!(benches, day1_benchmark);
criterion_main!(benches);
