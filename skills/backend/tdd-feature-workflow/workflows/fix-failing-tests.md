# Fix Failing Tests

## 1. Classify failure

- [ ] Compile failure
- [ ] Assertion failure
- [ ] Setup failure
- [ ] Flaky/environment failure
- [ ] Changed expected behavior
- [ ] Missing `#[cfg(test)] mod *_test;` wiring
- [ ] Test file placed at the wrong level

## 2. Decide action

- [ ] If production behavior is wrong, fix production code.
- [ ] If test expects outdated behavior, update the test only after confirming the new
      behavior is intended.
- [ ] If test is brittle implementation-detail testing, rewrite it to behavior-focused
      testing.
- [ ] If failure is infra-related, move it to integration test setup or isolate the
      dependency.
- [ ] If a source-level `*_test.rs` file is not running, check whether `#[cfg(test)] mod
      *_test;` is missing.
- [ ] If an integration helper was placed in `tests/common.rs`, consider moving it to
      `tests/common/mod.rs`.

## 3. Verify

- [ ] Run the failing test.
- [ ] Run related tests.
- [ ] Run `cargo test --all-features`.
