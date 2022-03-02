use criterion::{black_box, criterion_group, criterion_main, Criterion};
use safesecret::safe_secret;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("safe_secret", |b| {
        b.iter(|| {
            safe_secret(black_box(
                "8\n1 ? 5 + 0 ? -2 - -3 * -2 ? 3 * 1 -".to_owned(),
            ))
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
