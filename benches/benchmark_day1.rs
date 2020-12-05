use criterion::{criterion_group, criterion_main, Criterion};

use advent2020::get_string;

use advent2020::day1;

pub fn benchmark(c: &mut Criterion) {
    let expenses = day1::get_data(get_string("day1.txt"));
    c.bench_function("day1::main1", |b| {
        b.iter(|| day1::calc(expenses.to_vec(), 2))
    });
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
