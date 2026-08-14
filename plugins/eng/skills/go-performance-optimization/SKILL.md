---
name: go-performance-optimization
description: Optimizes Go backend performance with a measurement-first loop — benchmark (testing.B + benchstat), profile (pprof, execution trace), identify hot paths, reduce allocation/GC pressure, fix goroutine leaks and lock/channel contention, speed up pgx/sqlc/repository queries, tune Fiber/fasthttp handlers, raise worker/queue throughput, then verify before/after. Use when latency, throughput, memory, CPU, GC, goroutines, locking, or a hot path is the goal. Do not use for normal feature work, speculative optimization, style cleanup, CI/CD, or general code review.
---

# Go Performance Optimization

## Use this when

- API latency, throughput, memory, CPU, or GC time is the explicit goal.
- A DB/repository query is slow.
- Worker/queue throughput is poor.
- Goroutine count grows, or a goroutine leak is suspected.
- Lock or channel contention is suspected.
- Allocation or GC pressure in a hot path is suspected.
- A benchmark shows regression or a profile shows a hot path.
- The user asks to optimize performance.

## Do not use this when

- Implementing a normal feature → use `coke-eng:go-clean-architecture` (plus a TDD skill).
- Speculative optimization with no measured goal.
- Code style cleanup, CI/CD setup, deployment, security review, or general code review.
- Cloud infrastructure provisioning.

## Core rules

- Measure first; no optimization without a baseline.
- Match the measurement to the symptom: if it only hurts under load, or extra cores do not help, the baseline must be concurrent (`b.RunParallel`, an open-loop load harness, or a mutex/block profile). A serial benchmark cannot see contention, and it makes the correct fix look worthless.
- Optimize hot paths only — the uber-go guide scopes its own performance rules to hot paths, and so does this skill.
- Make the smallest safe change and re-measure; one change at a time.
- Compare benchmarks with `benchstat` across ≥10 runs; do not claim a win from a single run or when benchstat reports `~`.
- Preserve architecture boundaries — if an optimization needs a layer change, propose it and wait for approval.
- Keep correctness tests passing throughout (`go test -race ./...`).
- Do not introduce `unsafe` by default.
- Report tradeoffs, not just speedups.

## Architecture safety

```text
handlers -> usecases -> domain
infra -> domain interfaces
```

- Handlers stay thin (binding, DTO mapping, serialization, usecase calls).
- Usecases own orchestration and user-facing semantics.
- Domain stays pure — entities, value objects, invariants, repository interfaces.
- Infra owns SQL, sqlc-generated code, pool access, IO, and DB-specific optimization.
- DTOs never reach domain; sqlc-generated types never leak past `internal/infra/postgres`.

See `references/architecture-safe-optimization.md`.

## Optimization loop

1. Define the performance goal (latency, throughput, memory, CPU, p95/p99, GC time, etc.).
2. Identify the workload that produces the symptom.
3. Measure a baseline (benchmark, load test, or production profile).
4. Profile if the bottleneck is unclear — CPU, heap, goroutine, block, or mutex profile; execution trace when pprof looks fine but latency is bad.
5. Pick the smallest safe optimization for the layer that owns the bottleneck.
6. Run correctness tests (`go test -race ./...`).
7. Re-run the benchmark or profile under the same workload.
8. Compare before/after with `benchstat` or profile diffs.
9. Summarise tradeoffs and risks.

## Workflows

| Workflow                                            | Use for                                           |
| --------------------------------------------------- | ------------------------------------------------- |
| `workflows/define-performance-goal.md`              | Turning a vague complaint into a measurable goal  |
| `workflows/benchmark-hot-path.md`                   | Writing or running a hot-path benchmark           |
| `workflows/profile-and-identify-bottleneck.md`      | Profiling to find the bottleneck                  |
| `workflows/optimize-domain-usecase.md`              | Domain or usecase-level optimization              |
| `workflows/optimize-repository-db.md`               | Repository, query, or database-level optimization |
| `workflows/optimize-goroutine-worker.md`            | Goroutine, channel, worker, and leak fixes        |
| `workflows/optimize-allocation.md`                  | Allocation and GC-pressure reduction              |
| `workflows/add-performance-regression-benchmark.md` | Locking in the gain with a regression benchmark   |
| `workflows/verify-performance-change.md`            | Verifying the change before reporting done        |

## Load more detail

| Decision                                            | Reference                                      |
| --------------------------------------------------- | ---------------------------------------------- |
| Core performance principles                         | `references/performance-principles.md`         |
| Measurement-first discipline                        | `references/measure-first.md`                  |
| Profiling tools (pprof, trace, flamegraphs)         | `references/go-profiling-tools.md`             |
| testing.B benchmarks + benchstat                    | `references/go-benchmarking.md`                |
| Allocation reduction and escape analysis            | `references/allocation-and-escape.md`          |
| GC tuning and memory limits (GOGC, GOMEMLIMIT)      | `references/gc-and-memory.md`                  |
| Goroutines, channels, leaks, bounded concurrency    | `references/goroutine-concurrency.md`          |
| Locking and shared state                            | `references/locking-and-shared-state.md`       |
| Database / repository performance (pgx, sqlc)       | `references/database-performance.md`           |
| Fiber / fasthttp specifics                          | `references/fiber-performance.md`              |
| Worker / queue performance                          | `references/worker-queue-performance.md`       |
| Keeping architecture boundaries during optimization | `references/architecture-safe-optimization.md` |
| Common performance smells                           | `references/performance-smells.md`             |

## Templates

- `templates/benchmark_test.go`, `templates/performance-investigation.md`,
  `templates/api-latency-benchmark.md`, `templates/repository-query-benchmark.md`,
  `templates/worker-throughput-benchmark.md`, `templates/profiling-report.md`,
  `templates/before-after-report.md`, `templates/optimization-summary.md`.

## Related skills

- `coke-eng:go-clean-architecture` — layer structure that must be preserved.
- `coke-eng:rust-performance-optimization` — the Rust counterpart this skill mirrors.

## Final verification

If source code changed:

```sh
go build ./...
go vet ./...
go test -race ./...
```

If a benchmark exists:

```sh
go test -bench=. -benchmem -count=10 ./path/to/pkg | tee new.txt
benchstat old.txt new.txt
```

## Definition of done

Summarise:

- Performance goal and baseline.
- Benchmark or profile command used.
- Bottleneck found.
- Optimization made and which layer it touched.
- Architecture boundaries preserved.
- Before / after results with benchstat delta or profile evidence.
- Tests run and benchmark run.
- Risks and follow-up.
