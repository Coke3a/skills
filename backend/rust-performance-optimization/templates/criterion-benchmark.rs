// Template destination:
// benches/<name>_benchmark.rs
//
// Add to Cargo.toml:
// [dev-dependencies]
// criterion = "<version>"
//
// [[bench]]
// name = "<name>_benchmark"
// harness = false

use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn benchmark_example(c: &mut Criterion) {
    c.bench_function("example hot path", |b| {
        b.iter(|| {
            // Call public or intended interface here.
            // Use black_box for inputs that must not be optimized away.
            black_box(())
        })
    });
}

criterion_group!(benches, benchmark_example);
criterion_main!(benches);
