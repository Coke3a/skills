# Test Smells

- Testing private methods directly
- Asserting implementation details instead of behavior
- Excessive mocks
- One giant test for many behaviors
- Tests with unclear names
- Tests that duplicate production logic
- Slow unit tests with database or network access
- Flaky tests depending on time or randomness
- Snapshot tests used as a substitute for understanding behavior
- Overly broad API tests that duplicate domain/usecase tests
- Tests that pass even when production code is removed
- `*_test.rs` files under `src/` that are not wired with `#[cfg(test)] mod *_test;`
- Creating domain, usecase, repository, and API tests all upfront before any
  implementation
- Using API tests to verify pure domain validation that should be tested in domain tests
