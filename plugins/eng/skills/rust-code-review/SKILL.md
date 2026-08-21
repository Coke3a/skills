---
name: rust-code-review
description: Reviews Rust backend changes against Clean Architecture boundaries, TDD test quality, Rust idioms, error handling, async/concurrency safety, performance footguns, security basics, and final verification. Use before finishing a feature, before opening or merging a PR, when refactoring across layers, when async/concurrent code changed, or when asked to audit. Do not use for initial scaffolding, TDD red/green/refactor, CI/CD setup, deployment, or full benchmarking/profiling.
---

# Rust Code Review

## Use this when

- Before finishing a feature or marking a task done.
- After generated or large-scale code changes.
- Before opening or merging a pull request.
- When refactoring mixes handler/usecase/domain/infra code.
- When tests, repositories, handlers, or async/background/concurrent code changed.
- When asked to review, audit, check correctness, or check skill compliance.

## Do not use this when

- Initial architecture scaffolding → use `coke-eng:rust-clean-architecture`.
- Running the TDD red/green/refactor loop → use `coke-eng:tdd-feature-workflow`.
- Setting up CI/CD or deployment → use `coke-eng:ci-cd-workflow`.
- Formatting-only changes.
- Full benchmarking, profiling, or load testing → use `coke-eng:performance-optimization`.

## Core rules

- Review against existing skill rules; do not duplicate their full implementation workflows.
- Cite the specific file and rule for each finding.
- Severities run highest to lowest in this priority order: correctness, architecture boundaries, error handling, concurrency/async safety, test quality, security/data safety, performance footguns, simplicity, Rust idioms/naming, documentation, tool verification.
- Catch review-level Rust/Tokio footguns: blocking work in async code, locks held across `.await`, unbounded tasks/channels, ignored task failures, missing cancellation/shutdown, DB pool exhaustion risks, excessive hot-path allocation/cloning, unsafe concurrent access.
- Do not perform benchmarking, profiling, or load testing here. Recommend a dedicated performance workflow when impact is uncertain.
- Never fabricate command results.

## Workflow

1. Pick the workflow file matching the situation (see Workflows below).
2. Load only the reference files needed for the scope under review.
3. Verify or request the three final commands (see Final verification).
4. Produce findings using `templates/review-finding.md` and a summary using `templates/review-report.md` or `templates/final-review-summary.md`.

## Workflows

| Workflow                                          | Use for                                              |
| ------------------------------------------------- | ---------------------------------------------------- |
| `workflows/review-change.md`                      | Full change review                                   |
| `workflows/review-feature-before-finish.md`       | Before claiming a feature is done                    |
| `workflows/review-failing-implementation.md`      | Reviewing a failing implementation                   |
| `workflows/apply-review-fixes.md`                 | Applying findings from a review                      |

## Load more detail

| Scope                                       | Reference                                       |
| ------------------------------------------- | ----------------------------------------------- |
| Review priorities and severity              | `references/review-priorities.md`               |
| Core review principles                      | `references/review-principles.md`               |
| Clean Architecture boundary review          | `references/clean-architecture-review.md`       |
| Repository/Diesel review                    | `references/repository-review.md`               |
| Handler/API review                          | `references/handler-api-review.md`              |
| Error handling review                       | `references/error-handling-review.md`           |
| TDD test review                             | `references/tdd-test-review.md`                 |
| Async/concurrency and performance footguns  | `references/performance-concurrency-review.md`  |
| Security/data safety review                 | `references/security-review.md`                 |
| Rust quality checklist                      | `references/rust-quality-checklist.md`          |
| Common review smells                        | `references/review-smells.md`                   |
| Review comment style                        | `references/review-comment-style.md`            |

## Templates

- `templates/review-report.md` — full review summary (status, findings by area, risks).
- `templates/review-finding.md` — single finding format.
- `templates/review-comment.md` — line-level review comment.
- `templates/final-review-summary.md` — short, end-of-review summary.

## Related skills

- `coke-eng:rust-clean-architecture` — owns layer structure, naming, error flow, repository shape. This skill checks compliance.
- `coke-eng:tdd-feature-workflow` — owns the TDD loop and test-scope decision. This skill checks test quality.
- `coke-eng:performance-optimization` — when an issue needs measured impact, defer to it.
- `coke-eng:ci-cd-workflow` — owns automation that runs the final verification commands.

## Final verification

When reviewing a downstream Rust project, run or request:

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features
```

If the current repo is a skills repo only and has no Cargo project, report these as downstream verification commands.

## Definition of done

- Findings are concrete, cite file/line, name the violated rule, and propose a fix.
- Severity is assigned using `references/review-priorities.md`.
- Final summary uses `templates/review-report.md` (or `templates/final-review-summary.md` for a short review).
- The three final commands were run or were explicitly requested with the reason.
