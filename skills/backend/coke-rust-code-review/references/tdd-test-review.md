# TDD Test Review

Use this reference to check whether tests followed `coke-tdd-feature-workflow`.

## Principles

- Tests verify behavior, not implementation details.
- Prefer one behavior per test.
- Use the smallest useful test level.
- Do not create all test levels upfront.
- Avoid duplicating the same business rule across every layer unless each test adds unique
  confidence.

## Placement

- Domain tests go beside domain code as `src/domain/.../*_test.rs`.
- Usecase tests go beside usecase code as `src/usecases/.../*_test.rs`.
- Repository integration tests go under `tests/repositories/*_test.rs`.
- Handler/API tests go under `tests/api/*_test.rs`.
- Shared integration helpers go under `tests/common/mod.rs`.
- Every src-level `*_test.rs` file is wired with `#[cfg(test)] mod *_test;` in the parent module.

## Test Doubles

- Use fake repositories for usecase tests.
- Do not mock pure domain logic.
- Do not mock Diesel to prove database behavior.

## Repository and API Tests

- Add repository integration tests when persistence behavior, Diesel mapping, constraints,
  transactions, or DB error mapping changed.
- Add handler/API tests when HTTP contract, request/response mapping, route wiring, auth extraction,
  or API error mapping changed.
- API tests should focus on HTTP contract, not duplicate all business rules.

## Test Smells

- Names like `test_create`, `should_work`, or `test_all`.
- One giant test covering unrelated behaviors.
- Tests that duplicate production logic.
- Tests that assert private implementation details.
- Tests that pass if production logic is removed.
- Excessive mocks.
- Slow unit tests requiring DB/network.
- Tests added after implementation with no clear behavior they protect.

## Review Questions

- Would this test fail if behavior broke?
- Does this test verify behavior or implementation?
- Is this the smallest useful test level?
- Is this test duplicated at another layer?
- Is this test too slow for its level?
