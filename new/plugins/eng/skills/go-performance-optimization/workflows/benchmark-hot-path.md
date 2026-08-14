# Benchmark Hot Path

## 1. Choose the target

- [ ] Confirm the path is hot (profile evidence or a defined goal), not a guess
- [ ] Benchmark the narrowest public interface that contains the cost
- [ ] Decide unit benchmark (`testing.B`) vs load test (endpoint-level)

## 2. Write the benchmark

- [ ] Use `templates/benchmark_test.go`; `b.Loop()` on Go 1.24+, `b.ResetTimer()` + sink otherwise
- [ ] `b.ReportAllocs()` or run with `-benchmem`
- [ ] Realistic, deterministic input; document shape and size
- [ ] Table sub-benchmarks (`b.Run`) for size/implementation comparisons
- [ ] Do not benchmark setup; do not use trivially small inputs

## 3. Run the baseline

- [ ] `go test -bench=Benchmark{Name} -benchmem -count=10 ./path | tee old.txt`
- [ ] Quiet machine; note Go version and relevant env (GOMAXPROCS, GOGC)
- [ ] Sanity-check numbers (an impossibly fast result usually means eliminated code)

## 4. After the change

- [ ] Same command `| tee new.txt`
- [ ] `benchstat old.txt new.txt`; report delta and p-value
- [ ] `~` from benchstat = no significant change — report honestly
