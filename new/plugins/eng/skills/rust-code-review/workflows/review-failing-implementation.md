# Review a Failing Implementation

Use when tests, clippy, build, or behavior is failing.

1. Classify failure

- [ ] Compile failure.
- [ ] Clippy failure.
- [ ] Test failure.
- [ ] Runtime behavior failure.
- [ ] Architecture mismatch.
- [ ] Async/concurrency issue.
- [ ] Environment/setup issue.

2. Find likely layer

- [ ] Domain invariant issue.
- [ ] Usecase orchestration issue.
- [ ] Repository/Diesel issue.
- [ ] Handler/API mapping issue.
- [ ] Test setup issue.
- [ ] Error conversion issue.
- [ ] Async runtime issue.
- [ ] Locking/concurrency issue.
- [ ] Background task/worker issue.

3. Fix strategy

- [ ] Fix production code if behavior is wrong.
- [ ] Fix test if it asserts wrong/outdated behavior.
- [ ] Move logic to the correct layer if boundary is violated.
- [ ] Add missing error conversion if error flow is broken.
- [ ] Add missing #[cfg(test)] test module wiring if src-level tests are not discovered.
- [ ] Fix lock scope or async blocking if runtime/concurrency issue is found.
- [ ] Add cancellation/error handling if task lifecycle is broken.
- [ ] Do not silence clippy without a reason.

4. Verify

- [ ] Run the narrow failing test or command.
- [ ] Run related tests.
- [ ] Run final verification commands if possible.
