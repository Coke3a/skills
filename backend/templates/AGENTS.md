# AGENTS.md

## Project

Rust backend project using Clean Architecture.

## Available Backend Skills

Use these skills when relevant:

- **rust-clean-coke-architecture-patterns**:
  Use for layer structure, naming, error flow, repository traits, Diesel repository implementations, and handler/usecase/domain boundaries.

- **tdd-feature-workflow**:
  Use for new behavior, bug fixes, acceptance criteria, test scope selection, red/green/refactor, and Rust test file placement.

- **rust-code-review**:
  Use before finishing a feature or PR to review correctness, architecture, tests, Rust quality, error handling, async/concurrency, security, and verification readiness.

- **rust-ci-cd**:
  Use for GitHub Actions, Rust CI checks, Docker/image builds, deployment workflows, migrations, smoke tests, rollback, permissions, secrets, and OIDC.

- **rust-performance-optimization**:
  Use only when there is a performance goal, hot path, benchmark/profile need, suspected bottleneck, or performance regression.

## Required Commands Before Finishing Code Changes

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features
```

## Development Rules

- Keep handlers thin.
- Keep business logic in domain/usecases.
- Keep repository traits in domain.
- Keep Diesel implementations in infra.
- Do not leak DTOs into domain.
- Do not leak Diesel rows outside infra.
- Add or update tests for changed behavior.
- Do not create all test levels upfront.
- Do not bypass architecture boundaries for performance.
- Do not fabricate command results.

## Feature Workflow

1. Use tdd-feature-workflow to define acceptance criteria and the smallest useful first test.
2. Use rust-clean-coke-architecture-patterns to place code in the correct layers.
3. Implement one behavior at a time.
4. Run tests.
5. Use rust-code-review before finishing.

## Performance Workflow

1. Define performance goal.
2. Measure baseline.
3. Identify hot path.
4. Optimize smallest safe change.
5. Preserve architecture boundaries.
6. Re-measure.
7. Use rust-code-review.

## CI/CD Workflow

Use rust-ci-cd for CI, build, deploy, smoke test, rollback, and workflow hardening.
