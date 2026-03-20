use criterion::{criterion_group, criterion_main, Criterion};

fn lexer_benchmark(c: &mut Criterion) {
    c.bench_function("lexer", |b| b.iter(|| 1 + 1));
}

criterion_group!(benches, lexer_benchmark);
criterion_main!(benches);
