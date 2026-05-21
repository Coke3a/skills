---
name: coke-rust-performance-optimization
description: Optimizes Rust backend performance with a measurement-first loop — benchmark, profile, identify hot paths, reduce allocation/clone, tune async/Tokio, fix lock contention, speed up DB/repository queries, raise worker/queue throughput, then verify before/after. Use when latency, throughput, memory, CPU, async runtime, locking, or a hot path is the goal. Do not use for normal feature work, speculative optimization, style cleanup, CI/CD, or general code review.
---

# Rust Performance Optimization

## Use this when

- API latency, throughput, memory, or CPU is the explicit goal.
- A DB/repository query is slow.
- Worker/queue throughput is poor.
- Async runtime appears blocked, starved, or stuck.
- Lock contention is suspected.
- Allocation/clone in a hot path is suspected.
- A benchmark shows regression or a profile shows a hot path.
- The user asks to optimize performance.

## Do not use this when

- Implementing a normal feature → use `coke-rust-clean-architecture` + `coke-tdd-feature-workflow`.
- Speculative optimization with no measured goal.
- Code style cleanup, CI/CD setup, deployment, security review, or general code review.
- Cloud infrastructure provisioning.

## Core rules

- Measure first; no optimization without a baseline.
- Optimize hot paths only.
- Make the smallest safe change and re-measure.
- Preserve architecture boundaries — if an optimization needs a layer change, propose it and wait for approval.
- Keep correctness tests passing throughout.
- Do not introduce `unsafe` by default.
- Report tradeoffs, not just speedups.

## Architecture safety

```text
handlers -> usecases -> domain
infra -> domain traits
```

- Handlers stay thin (HTTP parsing, DTO mapping, serialization, usecase calls).
- Usecases own orchestration and user-facing semantics.
- Domain stays pure — entities, value objects, invariants, repository traits.
- Infra owns DB queries, Diesel rows, pool access, IO, and DB-specific optimization.
- DTOs never reach domain; row structs never leak past infra.

See `references/architecture-safe-optimization.md`.

## Optimization loop

1. Define the performance goal (latency, throughput, memory, CPU, p95/p99, etc.).
2. Identify the workload that produces the symptom.
3. Measure a baseline.
4. Profile if the bottleneck is unclear.
5. Pick the smallest safe optimization for the layer that owns the bottleneck.
6. Run correctness tests.
7. Re-run the benchmark or profile.
8. Compare before/after.
9. Summarise tradeoffs and risks.

## Workflows

| Workflow                                                | Use for                                              |
| ------------------------------------------------------- | ---------------------------------------------------- |
| `workflows/define-performance-goal.md`                  | Turning a vague complaint into a measurable goal     |
| `workflows/benchmark-hot-path.md`                       | Writing or running a hot-path benchmark              |
| `workflows/profile-and-identify-bottleneck.md`          | Profiling to find the bottleneck                     |
| `workflows/optimize-domain-usecase.md`                  | Domain or usecase-level optimization                 |
| `workflows/optimize-repository-db.md`                   | Repository, query, or database-level optimization    |
| `workflows/optimize-async-worker.md`                    | Async / Tokio / worker optimization                  |
| `workflows/optimize-allocation-clone.md`                | Allocation and clone reduction                       |
| `workflows/add-performance-regression-benchmark.md`     | Locking in the gain with a regression benchmark      |
| `workflows/verify-performance-change.md`                | Verifying the change before reporting done           |

## Load more detail

| Decision                                            | Reference                                           |
| --------------------------------------------------- | --------------------------------------------------- |
| Core performance principles                         | `references/performance-principles.md`              |
| Measurement-first discipline                        | `references/measure-first.md`                       |
| Profiling tools (flamegraph, samply, etc.)          | `references/rust-profiling-tools.md`                |
| Criterion + microbenchmarks                         | `references/rust-benchmarking.md`                   |
| Allocation and clone reduction                      | `references/allocation-and-clone.md`                |
| Bounds checks and hot loops                         | `references/bounds-checks-and-hot-loops.md`         |
| Async / Tokio performance                           | `references/async-tokio-performance.md`             |
| Locking and shared state                            | `references/locking-and-shared-state.md`            |
| Database / repository performance                   | `references/database-performance.md`                |
| Worker / queue performance                          | `references/worker-queue-performance.md`            |
| Keeping architecture boundaries during optimization | `references/architecture-safe-optimization.md`      |
| Common performance smells                           | `references/performance-smells.md`                  |

## Templates

- `templates/performance-investigation.md`, `templates/api-latency-benchmark.md`, `templates/repository-query-benchmark.md`, `templates/worker-throughput-benchmark.md`, `templates/profiling-report.md`, `templates/before-after-report.md`, `templates/optimization-summary.md`, and a Criterion benchmark template if the project uses one.

## Related skills

- `coke-rust-clean-architecture` — layer structure that must be preserved.
- `coke-tdd-feature-workflow` — correctness and regression tests.
- `coke-rust-code-review` — final code review including async/concurrency review.
- `coke-rust-ci-cd` — running benchmarks in CI when appropriate.

## Final verification

If source code changed:

```sh
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features
```

If a benchmark exists:

```sh
cargo bench
```

## Definition of done

Summarise:

- Performance goal and baseline.
- Benchmark or profile command used.
- Bottleneck found.
- Optimization made and which layer it touched.
- Architecture boundaries preserved.
- Before / after results.
- Tests run and benchmark run.
- Risks and follow-up.
