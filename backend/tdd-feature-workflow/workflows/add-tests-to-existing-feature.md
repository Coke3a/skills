# Add Tests to Existing Behavior

## 1. Identify current behavior

- [ ] Read the current implementation.
- [ ] Identify the public or intended contract.
- [ ] Identify expected behavior from docs, API, or code usage.
- [ ] Avoid locking down accidental implementation details.

## 2. Choose the smallest useful test

- [ ] Do not create all test levels upfront.
- [ ] Domain behavior -> `src/domain/.../*_test.rs`.
- [ ] Usecase behavior -> `src/usecases/.../*_test.rs`.
- [ ] Repository behavior -> `tests/repositories/*_test.rs`.
- [ ] API behavior -> `tests/api/*_test.rs`.
- [ ] Add `#[cfg(test)] mod *_test;` for source-level test files.

## 3. Characterization test

- [ ] Add tests for current externally observable behavior.
- [ ] Prefer public API or intended interface.
- [ ] Keep tests behavior-focused.
- [ ] Run tests and confirm they pass.

## 4. Change behavior

- [ ] Add a failing test for the desired new behavior.
- [ ] Implement the change.
- [ ] Run the narrow test.
- [ ] Run `cargo test --all-features`.

## 5. Refactor

- [ ] Refactor only after behavior is protected.
- [ ] Avoid changing behavior and refactoring at the same time.
- [ ] Remove tests that only lock implementation details.
