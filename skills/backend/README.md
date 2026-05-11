# Backend Skills Playbook

## Skills

- **rust-clean-coke-architecture-patterns**: Owns Rust Clean Architecture layer rules, file
  structure, naming, error flow, repository trait pattern, Diesel repository implementation pattern,
  and handler/usecase/domain boundaries.
- **tdd-feature-workflow**: Owns TDD workflow, acceptance criteria, red/green/refactor cycle, test
  scope selection, and Rust test file placement.
- **rust-code-review**: Owns correctness review, architecture review, TDD test review, Rust quality
  checks, error handling review, async/concurrency review, security review, and review findings.
- **rust-ci-cd**: Owns GitHub Actions, Rust CI checks, Docker builds, deployment workflows,
  migrations, smoke tests, rollback guidance, secrets, permissions, and OIDC hardening.
- **rust-performance-optimization**: Owns measure-first performance optimization, benchmark/profile
  workflow, hot-path identification, async/Tokio performance, allocation/clone optimization,
  DB/repository performance, and before/after reporting while preserving architecture boundaries.

## Recommended Workflow

### Project start

1. Use `rust-ci-cd` to set up CI.
2. Use `rust-clean-coke-architecture-patterns` to create the architecture skeleton.
3. Use `rust-code-review` before first commit.

### Feature development

1. Use `tdd-feature-workflow` to define behavior and test order.
2. Use `rust-clean-coke-architecture-patterns` for layer/file/error/repository structure.
3. Implement one behavior at a time.
4. Use `rust-code-review` before finishing.

### Bug fix

1. Use `tdd-feature-workflow` to add regression test first.
2. Fix the smallest correct behavior.
3. Use `rust-code-review`.

### CI/CD

1. Use `rust-ci-cd` for GitHub Actions, Docker, deployment, migration, smoke test, and rollback.

### Performance

1. Use `rust-performance-optimization` only when there is a performance goal, hot path,
   benchmark/profile, or suspected bottleneck.
2. Use a measure-first workflow.
3. Do not break clean architecture boundaries.

## Common Prompts

### New project

"Use rust-ci-cd to set up initial CI, then use rust-clean-coke-architecture-patterns to create a
minimal Rust backend clean architecture skeleton. Before finishing, use rust- code-review."

### New feature

"Use tdd-feature-workflow together with rust-clean-coke-architecture-patterns to implement this
feature one behavior at a time. Do not create all test levels upfront. Before finishing, use
rust-code-review."

### Code review

"Use rust-code-review to review the current changes for correctness, architecture boundaries, TDD
test quality, Rust idioms, error handling, async/concurrency safety, security, and verification
readiness."

### Performance

"Use rust-performance-optimization with rust-clean-coke-architecture-patterns. Measure first,
identify the hot path, optimize the smallest safe change, preserve architecture boundaries, then
re-measure."

### CI/CD

"Use rust-ci-cd to create or fix GitHub Actions workflows for Rust CI, Docker build,
staging/production deploy, migrations, smoke tests, rollback, and secrets/permissions."

## Required Verification

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features
```

For CI/CD projects also:

```bash
cargo build --release
```

## Notes

- Do not use every skill for every task.
- Use skills by phase.
- Architecture skill is not TDD.
- TDD skill is not CI/CD.
- Code review skill is a quality gate.
- Performance optimization is on-demand and measurement-first.
- CI/CD skill owns automation and deployment safety.
