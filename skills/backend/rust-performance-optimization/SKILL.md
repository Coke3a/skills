---
name: rust-performance-optimization
description:
  Use when optimizing Rust backend performance with measurement-first workflow. Guides
  benchmark/profiling, hot path identification, allocation/clone reduction, async/Tokio performance,
  lock contention, DB/repository performance, worker/queue throughput, and before/after reporting
  while preserving rust-clean-coke-architecture-patterns boundaries.
---

# Rust Performance Optimization

## Purpose

Use this skill to optimize Rust backend performance without breaking clean architecture. Prefer
measured, small, layer-correct changes over speculative rewrites.

## When to Use

- API latency is too high.
- Throughput is too low.
- Memory usage is too high.
- CPU usage is too high.
- DB/repository query is slow.
- Worker/queue throughput is poor.
- Async runtime appears blocked or starved.
- Lock contention is suspected.
- Allocation/clone hot path is suspected.
- User asks to optimize performance.
- Benchmark shows regression.
- Profile shows hot path.

## When Not to Use

- Normal feature implementation.
- Speculative optimization with no performance goal.
- Code style cleanup.
- CI/CD setup.
- General code review.
- Security review.
- Deployment.

## Core Rules

- Measure first.
- Optimize hot paths only.
- Make the smallest safe change.
- Preserve architecture boundaries.
- Keep correctness tests passing.
- Re-measure after changes.
- Do not introduce unsafe code by default.
- Do not optimize by moving logic into the wrong layer.
- Report tradeoffs.

## Companion Skills

- `rust-clean-coke-architecture-patterns` owns layer structure, dependency direction, naming, error
  flow, repository traits, and Diesel implementation patterns.
- `tdd-feature-workflow` owns correctness and regression test workflow.
- `rust-code-review` owns final code review, security review, async/concurrency review, and
  architecture review.
- `rust-ci-cd` owns CI/CD and deployment automation.

## Default Optimization Loop

1. Define performance goal.
2. Identify workload.
3. Measure baseline.
4. Profile if bottleneck is unclear.
5. Choose smallest safe optimization.
6. Preserve architecture boundaries.
7. Run correctness tests.
8. Re-run benchmark/profile.
9. Compare before/after.
10. Summarize tradeoffs and risks.

## Architecture Safety

```text
handlers -> usecases -> domain
infra -> domain traits
```

- Handlers stay thin and only own HTTP parsing, DTO mapping, serialization, and usecase calls.
- Usecases own orchestration and user-facing semantics.
- Domain stays pure and owns entities, value objects, invariants, pure rules, and repository traits.
- Infra owns DB queries, Diesel rows, pool access, IO, and DB-specific optimization.
- Repository traits remain the boundary.
- DTOs must not leak into domain.
- Diesel row structs must not leak into domain or handlers.
- Do not bypass layers for performance without explicit approval.
- If an optimization needs an architecture change, propose it first and wait for approval.

## Workflows

- `workflows/define-performance-goal.md`
- `workflows/benchmark-hot-path.md`
- `workflows/profile-and-identify-bottleneck.md`
- `workflows/optimize-domain-usecase.md`
- `workflows/optimize-repository-db.md`
- `workflows/optimize-async-worker.md`
- `workflows/optimize-allocation-clone.md`
- `workflows/add-performance-regression-benchmark.md`
- `workflows/verify-performance-change.md`

## References

Load the narrow reference needed for the current bottleneck:

- `references/performance-principles.md`
- `references/measure-first.md`
- `references/rust-profiling-tools.md`
- `references/rust-benchmarking.md`
- `references/allocation-and-clone.md`
- `references/bounds-checks-and-hot-loops.md`
- `references/async-tokio-performance.md`
- `references/locking-and-shared-state.md`
- `references/database-performance.md`
- `references/worker-queue-performance.md`
- `references/architecture-safe-optimization.md`
- `references/performance-smells.md`

## Templates

- `templates/performance-investigation.md`
- `templates/criterion-benchmark.rs`
- `templates/api-latency-benchmark.md`
- `templates/repository-query-benchmark.md`
- `templates/worker-throughput-benchmark.md`
- `templates/profiling-report.md`
- `templates/before-after-report.md`
- `templates/optimization-summary.md`

## Final Verification

If source code changed:

```sh
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features
```

If benchmark exists:

```sh
cargo bench
```

## Final Response Format

Summarize:

- Performance goal.
- Baseline.
- Benchmark/profile command.
- Bottleneck found.
- Optimization made.
- Architecture boundaries preserved.
- Before/after result.
- Tests run.
- Benchmark run.
- Risks/follow-up.

## Out of Scope

This skill does not define:

- Rust clean architecture structure.
- TDD workflow.
- CI/CD pipeline setup.
- Deployment.
- Broad code review.
- Security audit.
- Load testing infrastructure.
- Cloud infrastructure provisioning.

Use dedicated companion skills for those areas.
