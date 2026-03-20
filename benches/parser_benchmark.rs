use criterion::{criterion_group, criterion_main, Criterion};

fn parser_benchmark(c: &mut Criterion) {
    c.bench_function("parser", |b| b.iter(|| 1 + 1));
}

criterion_group!(benches, parser_benchmark);
criterion_main!(benches);
