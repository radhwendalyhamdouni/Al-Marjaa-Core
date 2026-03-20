use criterion::{criterion_group, criterion_main, Criterion};

fn interpreter_benchmark(c: &mut Criterion) {
    c.bench_function("interpreter", |b| b.iter(|| 1 + 1));
}

criterion_group!(benches, interpreter_benchmark);
criterion_main!(benches);
