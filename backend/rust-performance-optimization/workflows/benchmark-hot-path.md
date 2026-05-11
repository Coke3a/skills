# Benchmark Hot Path

## 1. Choose benchmark target

- [ ] Pure function/domain logic -> Criterion benchmark.
- [ ] Usecase orchestration -> benchmark with fake repository if useful.
- [ ] Repository query -> integration benchmark or DB timing.
- [ ] API endpoint -> integration/load-style benchmark.
- [ ] Worker queue -> synthetic throughput benchmark.

## 2. Create benchmark

- [ ] Place benchmark under benches/.
- [ ] Use realistic inputs.
- [ ] Use black_box where needed.
- [ ] Avoid benchmarking unrelated setup.
- [ ] Keep benchmark deterministic.
- [ ] Document workload assumptions.

## 3. Run benchmark

- [ ] Run cargo bench or narrow benchmark.
- [ ] Save baseline result.
- [ ] Do not fabricate results.

## 4. Interpret

- [ ] Identify whether performance issue is reproduced.
- [ ] If not reproduced, adjust workload or use profiling/production metrics.
