# Measure First

Before changing code, define:

- What is slow.
- Where it is slow.
- How it is measured.
- What target matters.
- What workload represents the real use case.
- Whether the issue is latency, throughput, memory, CPU, GC time, DB time, queue lag, goroutine
  growth, or scheduler starvation.
- Whether this path is actually hot.
- How improvement will be verified.

## Benchmark vs Profile vs Trace

- Use benchmarks (`testing.B`) when the target is known and repeatable.
- Use profiling (pprof) when it is unclear where time, allocation, locking, or blocking is spent.
- Use the execution trace (`go tool trace`) when pprof looks fine but latency is bad — it exposes
  scheduler latency, blocked goroutines, and GC pauses per goroutine.
- Use production metrics/profiles when synthetic workloads cannot reproduce the issue.
- Synthetic benchmarks are useful for controlled comparisons but can miss real data shape,
  contention, cache behavior, and IO variance.
- For latency claims from load tests, report p95/p99 (never mean) and use a constant-arrival-rate
  tool (vegeta, k6) to avoid coordinated omission.

## Before / After

- Record baseline command/source and result (`| tee old.txt`).
- Re-run the same workload after the change (`| tee new.txt`).
- Compare with `benchstat old.txt new.txt` across ≥10 runs; `~` means statistically
  indistinguishable — report that honestly.
- Capture profiles *during* load runs so the profile reflects real hot paths.
- Do not fabricate results or claim improvement without before/after data.
- Avoid hard CI thresholds unless the execution environment is stable.
