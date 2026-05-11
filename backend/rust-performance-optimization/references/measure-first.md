# Measure First

Before changing code, define:

- What is slow.
- Where it is slow.
- How it is measured.
- What target matters.
- What workload represents the real use case.
- Whether the issue is latency, throughput, memory, CPU, DB time, queue lag, or runtime
  starvation.
- Whether this path is actually hot.
- How improvement will be verified.

## Benchmark vs Profile

- Use benchmarks when the target is known and repeatable.
- Use profiling when it is unclear where time, allocation, locking, IO, or DB cost is
  spent.
- Use production metrics when synthetic workloads cannot reproduce the issue.
- Synthetic benchmarks are useful for controlled comparisons but can miss real data
  shape, contention, cache behavior, and IO variance.

## Before / After

- Record baseline command/source and result.
- Re-run the same workload after the change.
- Note variance and uncertainty.
- Do not fabricate results or claim improvement without before/after data.
- Avoid hard CI thresholds unless the execution environment is stable.
