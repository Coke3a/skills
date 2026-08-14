---
name: rust-tdd-feature-workflow
description: Drives test-driven development for Rust backend features — turns acceptance criteria into the smallest useful domain, usecase, repository integration, or API test, then runs red/green/refactor. Use when adding behaviour, changing a usecase or handler contract, adding validation rules, or fixing a bug with a regression test. Do not use for layer architecture, CI/CD, performance benchmarking, or final code review.
---

# TDD Feature Workflow

## Use this when

- Adding a new behaviour or feature.
- Changing usecase, handler, or repository behaviour.
- Adding or changing validation rules.
- Fixing a bug with a regression test.
- Adding or changing error mapping behaviour.
- Characterising existing behaviour before a refactor.

## Do not use this when

- Designing layer structure or file layout → use `coke-eng:rust-clean-architecture`.
- Setting up GitHub Actions or deployment → use `coke-eng:rust-ci-cd`.
- Producing a final review report → use `coke-eng:rust-code-review`.
- Performance benchmarking or profiling → use `coke-eng:rust-performance-optimization`.
- Pure formatting, documentation, or architecture-only refactors with no behaviour change.

## Core rules

- Start from behaviour and acceptance criteria; one focused test per behaviour.
- Run red → green → refactor; refactor only when tests are green.
- Choose the smallest useful test level; do not create domain + usecase + repository + API tests upfront.
- Prefer public/intended interfaces over private implementation details.
- Avoid infrastructure in unit tests; fake only the boundaries (especially repository traits).
- Repository integration tests are added when persistence behaviour, Diesel mapping, constraints, transactions, or DB error mapping change.
- Handler/API tests are added when HTTP contract, DTO mapping, route wiring, auth extraction, or `ApiError` mapping change.
- Do not test the same business rule at every layer unless each level provides unique confidence.
- Put source-level Rust tests in sibling `*_test.rs` files and wire them with `#[cfg(test)] mod *_test;` in the parent module.
- Repository and API integration tests live under `tests/`, not in `src/tests/` or `test_process/`.

## TDD loop

1. Pick one acceptance criterion.
2. Choose the smallest useful test level (see `references/test-scope.md`).
3. Choose the test file location (see `references/test-file-placement.md`).
4. Write the failing test and wire any `*_test.rs` module if it lives under `src/`.
5. Run the narrowest test command.
6. Confirm it fails for the expected reason.
7. Write the smallest implementation that makes it pass.
8. Run the same test until green.
9. Refactor production and test code while green.
10. Run the relevant broader test set.
11. Repeat for the next criterion.

## Pairing with clean architecture

When used with `coke-eng:rust-clean-architecture`, that companion skill owns architecture, naming, error flow, and repository shape. This skill owns test order, level, placement, and the red/green/refactor loop:

- Domain tests protect entities, value objects, invariants, and pure rules.
- Usecase tests protect orchestration, permissions/ownership, and error semantics with a fake repository.
- Repository integration tests protect Diesel mapping and database behaviour.
- Handler/API tests protect HTTP contract, DTO mapping, and `ApiError`.

Do not let tests force handlers to hold business logic. Do not let architecture scaffolding skip behaviour tests.

## Load more detail

| Decision                                           | Reference                                       |
| -------------------------------------------------- | ----------------------------------------------- |
| Choosing the right test level                      | `references/test-scope.md`                      |
| `*_test.rs` placement and module wiring            | `references/test-file-placement.md`             |
| Mapping tests to clean-architecture layers         | `references/clean-architecture-test-mapping.md` |
| Idiomatic Rust test patterns                       | `references/rust-test-patterns.md`              |
| Core TDD principles                                | `references/tdd-principles.md`                  |
| Common test smells to avoid                        | `references/test-smells.md`                     |

## Workflows

| Workflow                                           | Use for                                                  |
| -------------------------------------------------- | -------------------------------------------------------- |
| `workflows/tdd-feature.md`                         | New feature implementation                               |
| `workflows/add-tests-to-existing-feature.md`       | Characterising existing behaviour before changing it     |
| `workflows/fix-failing-tests.md`                   | Diagnosing failing tests                                 |

## Templates

- `templates/acceptance-criteria.md` for planning behaviour to cover.
- `templates/test-summary.md` for the final report.

## Related skills

- `coke-eng:rust-clean-architecture` — layer structure, naming, error flow, repository shape.
- `coke-eng:rust-code-review` — final review against TDD and architecture rules.
- `coke-eng:rust-ci-cd` — automation that runs these tests in CI.

## Definition of done

- Each acceptance criterion is covered by the smallest useful test at the right layer.
- Tests fail for the right reason before any implementation passes them.
- Source-level `*_test.rs` files are wired with `#[cfg(test)] mod *_test;` in the parent module.
- `cargo test --all-features` (or the narrowest command the change requires) passes.
- Behaviour intentionally not covered is listed in the final summary.
