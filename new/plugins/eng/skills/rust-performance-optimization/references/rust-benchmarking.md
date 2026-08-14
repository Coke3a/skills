# Rust Benchmarking

- Use Criterion for pure functions, domain logic, parsing, serialization, and implementation
  comparisons.
- Use `cargo bench` or the narrowest practical benchmark command.
- Place Criterion benchmarks under `benches/`.
- Add `[[bench]]` with `harness = false` when using Criterion.
- Use `black_box` to prevent unrealistic compiler elimination.
- Benchmark public or intended interfaces when possible.
- Use realistic inputs and deterministic workloads.
- Avoid benchmarking unrelated setup.
- Document workload assumptions and input size.
- Avoid noisy pass/fail thresholds in CI unless the environment is stable.

Examples:

- `benches/domain_validation_benchmark.rs`
- `benches/usecase_create_entity_benchmark.rs`
- `benches/repository_query_benchmark.rs`
- `benches/worker_throughput_benchmark.rs`
- `benches/serialization_benchmark.rs`
