# Implement a Feature with TDD

## 1. Clarify behavior

- [ ] Read user request and existing code.
- [ ] Write acceptance criteria.
- [ ] Identify success behavior.
- [ ] Identify validation errors.
- [ ] Identify not found/conflict/permission behavior.
- [ ] Identify infrastructure failure behavior.
- [ ] Identify which clean architecture layers are affected.

## 2. Choose the smallest useful test

- [ ] Do not create all test levels upfront.
- [ ] Choose the smallest test that proves the next behavior.
- [ ] Domain test for pure rules/value objects/entities.
- [ ] Usecase test for orchestration and error semantics.
- [ ] Repository integration test only for DB query/mapping/persistence behavior.
- [ ] Handler/API test only for HTTP contract behavior.
- [ ] Avoid UI/E2E unless the behavior is truly user journey critical.

## 3. Choose test file location

- [ ] Domain tests go beside domain code as `src/domain/.../*_test.rs`.
- [ ] Usecase tests go beside usecase code as `src/usecases/.../*_test.rs`.
- [ ] Repository integration tests go under `tests/repositories/*_test.rs`.
- [ ] Handler/API tests go under `tests/api/*_test.rs`.
- [ ] Shared integration helpers go under `tests/common/mod.rs`.
- [ ] For every source-level `*_test.rs` file, add `#[cfg(test)] mod *_test;` in the parent module.

## 4. Red

- [ ] Write the smallest failing test for one behavior.
- [ ] Keep test focused on observable behavior.
- [ ] Use Arrange/Act/Assert.
- [ ] Run the narrowest relevant test.
- [ ] Confirm it fails for the expected reason.
- [ ] Do not continue if the failure is unrelated or pre-existing.

## 5. Green

- [ ] Write the smallest production code needed.
- [ ] Do not over-generalize.
- [ ] Do not add unrelated abstractions.
- [ ] Do not add unrelated tests.
- [ ] Run the same test until it passes.

## 6. Refactor

- [ ] Refactor only while green.
- [ ] Remove duplication.
- [ ] Improve names.
- [ ] Keep tests readable.
- [ ] Do not change behavior during refactor.

## 7. Repeat

- [ ] Move to the next acceptance criterion.
- [ ] Add edge cases.
- [ ] Add error cases.
- [ ] Add integration/API tests only where they provide unique confidence.
- [ ] Avoid duplicating the same business rule across every layer.

## 8. Final verification

- [ ] Run `cargo test --all-features`.
- [ ] Summarize tests and protected behaviors.
