# Review a Rust Change

1. Understand scope
- [ ] Identify files changed.
- [ ] Identify intended behavior.
- [ ] Identify affected layers.
- [ ] Identify whether tests changed.
- [ ] Identify whether DB/API behavior changed.
- [ ] Identify whether async/background/concurrent behavior changed.
- [ ] Identify whether hot-path or per-request code changed.

2. Review correctness
- [ ] Check success path.
- [ ] Check validation/error paths.
- [ ] Check edge cases.
- [ ] Check ownership/authorization if relevant.
- [ ] Check concurrency/async behavior if relevant.

3. Review architecture
- [ ] handlers -> usecases -> domain.
- [ ] infra -> domain traits.
- [ ] No business logic in handlers.
- [ ] No Axum/Diesel/schema dependency in domain/usecases.
- [ ] Repository traits in domain.
- [ ] Repository implementations in infra.
- [ ] DTOs do not leak into domain.
- [ ] Row structs do not leak outside infra.

4. Review tests
- [ ] Changed behavior has tests.
- [ ] Tests are behavior-focused.
- [ ] Test level is appropriate.
- [ ] No all-levels-upfront test bloat.
- [ ] Domain/usecase tests are fast and isolated.
- [ ] Repository integration tests exist when persistence behavior changed.
- [ ] API tests exist when HTTP contract changed.
- [ ] src-level *_test.rs files are wired with #[cfg(test)].

5. Review Rust quality
- [ ] Naming follows Rust conventions.
- [ ] No unnecessary clones/allocations.
- [ ] No unwrap/expect in production without justification.
- [ ] Error handling preserves useful context.
- [ ] Public APIs and DTOs are intentional.

6. Review performance and concurrency
- [ ] No blocking operation inside async handlers/tasks unless isolated with spawn_blocking or project-approved strategy.
- [ ] CPU-heavy work is bounded and not run directly on async runtime worker threads.
- [ ] No blocking lock guard or borrow is held across .await.
- [ ] Lock scope is short and contention risk is acceptable.
- [ ] Shared mutable state uses Mutex/RwLock/channel/actor pattern intentionally.
- [ ] Channels/queues are bounded or have explicit backpressure strategy.
- [ ] Spawned tasks have error handling, cancellation, and shutdown strategy when needed.
- [ ] JoinHandle is not ignored when task failure matters.
- [ ] External IO has timeout/retry/backoff where appropriate.
- [ ] DB pool usage cannot be exhausted by unbounded concurrency.
- [ ] Hot paths avoid unnecessary clone(), to_string(), collect(), or allocation.
- [ ] Performance concerns are reported as footguns, not speculative micro-optimizations.

7. Review security/data safety
- [ ] No secrets/PII logged.
- [ ] No infra details leaked to API users.
- [ ] Ownership/auth checks are present where needed.
- [ ] Input validation exists at the right layer.
- [ ] No raw SQL unless explicitly justified.
- [ ] No IDOR-style access bug.
- [ ] Concurrent/background paths preserve tenant/user boundaries.

8. Verify
- [ ] Run or request:
      cargo fmt --all -- --check
      cargo clippy --all-targets --all-features -- -D warnings
      cargo test --all-features
- [ ] Do not fabricate command results.
- [ ] Report any command not run.

9. Report
- [ ] Summarize status.
- [ ] List findings by severity.
- [ ] Include suggested fixes.
- [ ] Mention what was not reviewed.
