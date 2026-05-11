---
name: rust-code-review
description: Use when reviewing Rust backend changes for correctness, Clean Architecture boundaries, TDD test quality, Rust idioms, error handling, repository/handler patterns, async/concurrency safety, performance footguns, security issues, and final verification readiness. Pair with rust-clean-coke-architecture-patterns and tdd-feature-workflow.
---

# Rust Code Review

## Purpose

Review Rust backend changes and produce actionable findings. Check whether implementation follows:

- Rust clean architecture rules
- TDD test quality rules
- Rust idioms and safety
- Error handling patterns
- Repository and handler boundaries
- Async/concurrency safety
- Performance footgun checks
- Security basics
- Final verification expectations

## When to Use

Use this skill before finishing a feature, after generated code, before opening or merging a PR, when refactoring mixed handler/usecase/domain/infra code, when tests/repositories/handlers changed, when async/background/concurrent code changed, or when the user asks to review, audit, check correctness, or check skill compliance.

## When Not to Use

Do not use this for initial architecture scaffolding, TDD red/green/refactor implementation, CI/CD setup, deployment, formatting-only changes, full benchmarking, profiling, or load testing.

## Review Priorities

1. Correctness
2. Architecture boundaries
3. Error handling and failure behavior
4. Concurrency and async safety
5. Test quality
6. Security/data safety
7. Performance footguns
8. Simplicity/maintainability
9. Rust idioms/naming
10. Documentation/comments
11. Tool verification

## Companion Skills

- `rust-clean-coke-architecture-patterns` owns layer structure, naming, error flow, repository trait patterns, and Diesel implementation patterns.
- `tdd-feature-workflow` owns TDD workflow, behavior-focused tests, test scope, and test placement.
- `rust-code-review` checks whether the change followed those skills. Do not duplicate their full implementation workflows.

## Workflows

- Full change review: use `workflows/review-change.md`.
- Before claiming a feature is done: use `workflows/review-feature-before-finish.md`.
- Failing implementation review: use `workflows/review-failing-implementation.md`.
- Applying review fixes: use `workflows/apply-review-fixes.md`.

Load only the references needed for the review scope. Use `references/review-priorities.md` for severity, `references/clean-architecture-review.md` for architecture, `references/tdd-test-review.md` for tests, and `references/performance-concurrency-review.md` for async/concurrency and performance footguns.

## Performance and Concurrency Scope

Catch review-level Rust/Tokio performance and concurrency footguns: blocking work in async code, locks across `.await`, unbounded tasks/channels, ignored task failures, missing cancellation/shutdown behavior, DB pool exhaustion risks, excessive hot-path allocation/cloning, and unsafe concurrent access patterns.

Do not perform benchmarking, profiling, or load testing. If impact is uncertain or workload-dependent, recommend a dedicated benchmark/profiling workflow.

## Final Verification

When reviewing a downstream Rust project, verify or ask the implementing agent to run:

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features
```

Do not fabricate command results. If the repository is only a skills repository and has no Cargo project, report that these are downstream verification commands for projects using the skill.

## Review Report Format

```markdown
# Review Summary

Overall status:
- Pass / Pass with comments / Needs changes / Blocked

Scope reviewed:
- ...

Commands run:
- ...

Commands not run:
- ...

Findings:
1. [Severity] Title
   - File:
   - Issue:
   - Why it matters:
   - Suggested fix:
   - Related rule:

Architecture:
- ...

Tests:
- ...

Rust quality:
- ...

Error handling:
- ...

Performance/concurrency:
- ...

Security/data safety:
- ...

Remaining risks:
- ...
```
