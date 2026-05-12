---
name: coke-tdd-feature-workflow
description:
  Use when adding or changing behavior with Test-Driven Development. Guides acceptance criteria,
  red-green-refactor, choosing the smallest useful test level, behavior-focused tests, Rust
  domain/usecase/repository/API tests, sibling *_test.rs placement, and test summaries. Pair with
  coke-rust-clean-architecture for layer structure.
---

# TDD Feature Workflow

## Purpose

Guide implementation through tests first, one behavior at a time. Use this skill to make acceptance
criteria explicit, choose the smallest useful test level, run the red/green/refactor loop, and
summarize the behavior protected by tests.

## When to Use

- Adding a new feature
- Changing business behavior
- Fixing a bug with a regression test
- Adding validation rules
- Changing usecase behavior
- Changing repository persistence behavior
- Changing handler/API contract
- Adding or changing error mapping behavior

## When Not to Use

- Pure formatting changes
- Documentation-only changes
- CI/CD setup
- Architecture-only refactor with no behavior change
- Performance benchmarking
- Exploratory prototype where behavior is not yet stable

## Core Rules

- Start from behavior and acceptance criteria.
- Write one focused test per behavior.
- Use red/green/refactor.
- Do not create all test levels upfront.
- Prefer public or intended interfaces over private implementation.
- Choose the narrowest useful test first.
- Avoid infrastructure in unit tests.
- Fake only boundaries, especially repository traits.
- Refactor only when tests are green.
- Put source-level Rust tests in sibling `*_test.rs` files and wire them with `#[cfg(test)]`.

## TDD Loop

1. Pick one acceptance criterion.
2. Choose the smallest useful test level.
3. Choose the test file location.
4. Write the failing test.
5. Wire the `*_test.rs` module if it is under `src/`.
6. Run the narrowest test command.
7. Confirm it fails for the expected reason.
8. Write the smallest implementation.
9. Run the same test until green.
10. Refactor production code and test code.
11. Run the relevant broader test set.
12. Repeat.

## Do Not Create All Test Levels Upfront

Do not create domain, usecase, repository integration, and API tests all at once. Start with the
smallest test that proves the next behavior or design decision.

Add repository integration tests only when persistence behavior, Diesel mapping, database
constraints, transactions, or database error mapping are introduced or changed. Add handler/API
tests only when HTTP contract, request/response DTO mapping, route wiring, auth extraction, or API
error mapping are introduced or changed.

Do not test the same business rule in every layer unless each test level provides unique confidence.

## Test Scope Selection

| Behavior type                       | Preferred test level                                    | Preferred location              |
| ----------------------------------- | ------------------------------------------------------- | ------------------------------- |
| Value object validation             | Domain unit test                                        | `src/domain/.../*_test.rs`      |
| Entity invariant                    | Domain unit test                                        | `src/domain/entities/*_test.rs` |
| Pure business rule                  | Domain unit test                                        | `src/domain/.../*_test.rs`      |
| Usecase orchestration               | Usecase test with fake repo                             | `src/usecases/.../*_test.rs`    |
| DomainError -> UsecaseError mapping | Usecase test                                            | `src/usecases/.../*_test.rs`    |
| RepoError -> UsecaseError mapping   | Usecase test with fake repo                             | `src/usecases/.../*_test.rs`    |
| Repository query/mapping            | Repository integration test                             | `tests/repositories/*_test.rs`  |
| Database constraint behavior        | Repository integration test                             | `tests/repositories/*_test.rs`  |
| HTTP request/response               | Handler/API test                                        | `tests/api/*_test.rs`           |
| ApiError/status mapping             | Handler/API test                                        | `tests/api/*_test.rs`           |
| Bug fix                             | Smallest regression test that would have caught the bug | depends on behavior             |
| Cross-layer critical flow           | Integration/API test                                    | `tests/api/*_test.rs`           |
| Visual/UI behavior                  | UI test, only for critical journeys                     | project-specific                |

## Test File Placement

- Use `src/domain/.../*_test.rs` for domain tests.
- Use `src/usecases/.../*_test.rs` for usecase tests.
- Use `tests/repositories/*_test.rs` for Diesel/database integration tests.
- Use `tests/api/*_test.rs` for Axum/API tests.
- Use `tests/common/mod.rs` for shared integration helpers.
- Declare every source-level `*_test.rs` file with `#[cfg(test)] mod *_test;` in the parent module.
- Do not create `src/tests/` or generic `test_process/` directories.

## Rust Clean Architecture Mapping

When used with `coke-rust-clean-architecture`, let that companion skill define
architecture, file structure, names, error flow, repository trait shape, Diesel implementation
shape, and handler -> usecase -> domain boundaries. In this workspace, the companion folder is
`skills/backend/coke-rust-clean-architecture/`.

Use this skill to decide test order, test level, test file placement, and the red/green/refactor
loop:

- Domain tests protect entities, value objects, invariants, and pure rules.
- Usecase tests protect orchestration, permissions, ownership, and error semantics.
- Repository integration tests protect Diesel/database behavior.
- Handler/API tests protect HTTP contract.

## Pairing with coke-rust-clean-architecture

When both architecture and TDD are needed:

1. Use coke-rust-clean-architecture to decide files, layers, names, error flow, and
   repository shape.
2. Use this skill to decide test order, test level, test file placement, and red/green/refactor
   loop.
3. Put domain/usecase tests beside source modules as `*_test.rs` files.
4. Put repository/API integration tests under `tests/`.
5. Do not let tests force handlers to contain business logic.
6. Do not let architecture scaffolding skip behavior tests.
7. Do not create all test levels upfront just because the architecture has multiple layers.

## Bundled Resources

- Use `workflows/tdd-feature.md` for new feature implementation.
- Use `workflows/add-tests-to-existing-feature.md` when characterizing existing behavior before
  changing it.
- Use `workflows/fix-failing-tests.md` when diagnosing failing tests.
- Use `references/test-file-placement.md` for Rust module wiring and placement rules.
- Use `references/test-scope.md` and `references/clean-architecture-test-mapping.md` to choose the
  right test level.
- Use `references/rust-test-patterns.md` for Rust examples.
- Use `references/test-smells.md` to catch brittle or over-broad tests.
- Use `templates/acceptance-criteria.md` and `templates/test-summary.md` for planning and final
  reporting.
- Use Rust templates only as starting points; adapt names and imports to the actual project.

## Final Verification

Required:

```bash
cargo test --all-features
```

Optional local narrowing:

```bash
cargo test <test_name>
cargo test --test <test_file>
```

The companion architecture skill may also require:

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features
```

## Final Response Format

Summarize:

- Acceptance criteria covered
- Tests added/changed
- Test locations
- Implementation added/changed
- Commands run
- Any behavior intentionally not covered
- Risks/follow-up

## Out of Scope

This skill does not define:

- Rust clean architecture layer structure
- Project file structure beyond test placement
- CI/CD pipeline setup
- GitHub Actions
- Code review process
- Deployment flow
- Performance benchmarking
- Complete frontend/mobile UI testing strategy

Use the dedicated skills for those areas when available.
