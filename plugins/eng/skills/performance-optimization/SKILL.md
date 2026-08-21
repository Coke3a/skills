---
name: performance-optimization
description: Diagnose and improve software performance with a measurement-first workflow across languages and stacks. Use for latency, throughput, CPU, memory, allocation, contention, I/O, query, concurrency, startup, or bundle-size problems when a baseline can be reproduced. Do not use for speculative tuning, ordinary feature work, or general code review.
---

# Performance Optimization

Improve an observed workload, not code that merely looks inefficient. Let the repository choose the language-specific tools; this skill owns the investigation and verification loop.

## Establish the experiment

Before changing code, determine:

- the user-visible or operational symptom;
- the representative workload and environment;
- the metric and target, such as p95 latency, requests/second, CPU time, peak memory, allocations, startup time, or output size;
- the canonical benchmark, load-test, or profiling commands already present in the repository.

If no reproducible measurement exists, create the smallest useful one. Match it to the symptom: a serial microbenchmark cannot demonstrate contention, and a synthetic function benchmark cannot establish end-to-end latency.

## Workflow

1. Record the baseline under a controlled workload.
2. Profile the relevant resource when the bottleneck is not already isolated.
3. Trace the hot path to the component that owns the cost.
4. Form one falsifiable hypothesis.
5. Make the smallest safe change that tests it.
6. Run correctness and safety checks appropriate to the changed code.
7. Repeat the same measurement and compare distributions, not anecdotes.
8. Keep the change only when the evidence supports it; report regressions and tradeoffs.
9. Add a regression benchmark or budget when it will be stable and useful in automation.

Change one material variable at a time. Warm up runtimes and caches when relevant, use enough samples to expose noise, and keep hardware, data, concurrency, and build mode comparable. Never claim a gain from a single noisy run.

## Choose tools from the project

Inspect manifests, scripts, task runners, CI, and existing benchmark suites before inventing commands. Prefer the project's established profiler and statistical comparison tools. Examples include Go benchmarks and pprof, Rust Criterion and flamegraphs, JavaScript performance marks and bundle analyzers, JVM profilers, database `EXPLAIN`, and system-level sampling. These are options, not required tools.

Read [references/investigation-guide.md](references/investigation-guide.md) when choosing a measurement or profiling strategy.

## Boundaries

Preserve public behavior, correctness, security, and architectural ownership unless the user approves a tradeoff. Do not move business logic across boundaries merely to improve a microbenchmark. Treat unsafe code, weakened durability, unbounded concurrency, cache invalidation, and increased operational complexity as explicit design decisions.

For database changes, verify the actual query plan and representative data. For concurrent code, check saturation, queueing, cancellation, leaks, and contention rather than throughput alone. For networked systems, distinguish application time from dependency and transport time.

## Definition of done

Report the workload, environment, baseline, bottleneck evidence, hypothesis, change, before/after result, correctness checks, uncertainty, and tradeoffs. State clearly when the result is inconclusive.
