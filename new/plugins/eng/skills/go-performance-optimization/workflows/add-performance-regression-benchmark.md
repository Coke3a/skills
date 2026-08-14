# Add Performance Regression Benchmark

Lock in a measured gain so it cannot silently regress.

## 1. Decide scope

- [ ] Benchmark the interface that was optimized (usecase method, mapping func, repository method)
- [ ] Keep the workload realistic and deterministic; document input shape in the benchmark

## 2. Add the benchmark

- [ ] Follow `templates/benchmark_test.go`; `b.ReportAllocs()` on
- [ ] Commit the baseline numbers in the benchmark file comment or the PR description
      (command + benchstat output)

## 3. CI considerations

- [ ] Prefer tracked trends over hard pass/fail thresholds — shared CI runners are noisy
- [ ] If a threshold is required, gate on large regressions only and use `-count` ≥ 10 with
      benchstat comparison
- [ ] Keep the benchmark fast enough to run in CI, or mark it for a nightly job

## 4. Verify

- [ ] `go test -bench=. -benchmem ./...` runs green locally
- [ ] The benchmark actually exercises the optimized path (revert the optimization locally once;
      the number must move)
