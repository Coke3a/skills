# Go Benchmarking

- Use `testing.B` benchmarks for pure functions, domain logic, parsing, serialization, mapping
  layers, and implementation comparisons.
- Place benchmarks in `_test.go` files next to the code (`func BenchmarkXxx(b *testing.B)`).
- Run the narrowest practical command:
  `go test -bench=BenchmarkName -benchmem -count=10 ./path/to/pkg`.
- Always include `-benchmem` (or `b.ReportAllocs()`): allocs/op is usually the first number to
  attack.
- On Go 1.24+, prefer `for b.Loop()` over `for i := 0; i < b.N; i++` — setup runs once and the
  compiler keeps benchmarked calls alive, eliminating dead-code-elimination traps and manual
  `sink` variables.
- On older Go, use `b.ResetTimer()` after expensive setup and a package-level `sink` assignment to
  prevent elimination.
- Compare with **benchstat**: `-count=10` before and after, then `benchstat old.txt new.txt`.
  Report the delta and p-value; `~` means no statistically significant change — say so.
- Benchmark public or intended interfaces when possible.
- Use realistic inputs and deterministic workloads; document workload assumptions and input size.
- Avoid benchmarking unrelated setup; avoid trivially small inputs that measure loop overhead.
- Avoid noisy pass/fail thresholds in CI unless the environment is stable.

## Serial benchmarks are blind to contention

A `testing.B` loop runs one operation at a time, so it cannot observe lock contention, channel
handoff, or scheduler starvation — the costs that only exist when requests overlap. On a serialized
service the serial number is identical before and after the correct fix, which makes the fix look
worthless and pushes you toward whatever cheap allocation is visible instead.

Whenever the symptom mentions concurrency — "fine in isolation, terrible under load", "more cores
don't help", "p99 is bad but p50 is fine" — the baseline must be concurrent:

```go
func BenchmarkServeParallel(b *testing.B) {
	srv := newServer()
	var n int64

	b.ReportAllocs()
	b.RunParallel(func(pb *testing.PB) {
		for pb.Next() {
			i := atomic.AddInt64(&n, 1) // vary input per iteration
			srv.handle(requestFor(i))
		}
	})
}
```

Confirm the diagnosis three ways rather than one: compare serial against `RunParallel`, compare
throughput at `GOMAXPROCS=1` against `GOMAXPROCS=8` (flat scaling is the signature of
serialization), and take a mutex or block profile. A closed-loop benchmark measures throughput; an
open-loop harness at a fixed arrival rate is what measures the tail latency users actually feel.

## Measurement hygiene

- Interleave A/B rounds instead of running all baseline samples then all after samples — machine
  state drifts, and interleaving cancels the drift.
- Nothing else heavy should run on the box. If load rises mid-run, say so in the report and label
  the affected numbers rather than quietly keeping them.
- Pin correctness with a golden test (hash the output) before touching anything, so every later
  number is comparable and any behavior change is caught immediately.

Examples:

- `internal/domain/valueobject/example_entity_name_bench_test.go`
- `internal/usecase/examplefeature/create_bench_test.go`
- `internal/infra/postgres/example_repository_bench_test.go` (integration, real or containerized DB)
- serialization / DTO-mapping benchmarks next to the mapping code
