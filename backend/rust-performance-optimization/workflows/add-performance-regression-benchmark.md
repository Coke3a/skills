# Add Performance Regression Benchmark

## 1. Choose benchmark

- [ ] Benchmark real hot behavior.
- [ ] Avoid benchmarking private implementation details if public/intended interface is available.
- [ ] Keep workload deterministic.
- [ ] Document input size and assumptions.

## 2. Add benchmark

- [ ] Add Criterion dev-dependency if project accepts it.
- [ ] Add [[bench]] with harness = false.
- [ ] Create benches/<name>_benchmark.rs.
- [ ] Use black_box where needed.

## 3. Run

- [ ] cargo bench.
- [ ] Record baseline.
- [ ] Do not add noisy pass/fail thresholds unless CI is configured for stable performance testing.
