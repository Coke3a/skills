# Review Feature Before Finish

Use this before the agent says the feature is done.

1. Confirm acceptance criteria

- [ ] List the intended behaviors.
- [ ] Check each behavior has implementation.
- [ ] Check each behavior has appropriate tests or a reason it is not tested.

2. Confirm architecture

- [ ] Check changed files are in correct layers.
- [ ] Check dependency direction.
- [ ] Check naming conventions.
- [ ] Check handler files are organized into app composition, shared utilities, and endpoint routers.
- [ ] Check `mod.rs` files are declaration-only with only `pub mod ...;`.
- [ ] Check error flow.
- [ ] Check repository pattern.
- [ ] Check usecases are grouped by feature/domain with action leaf files.
- [ ] Check external IO uses domain service traits and infra implementations.
- [ ] Check entities and value objects live in the expected domain subfolders.

3. Confirm tests

- [ ] Domain rules covered in domain tests.
- [ ] Usecase orchestration covered in usecase tests.
- [ ] DB behavior covered in repository integration tests if DB changed.
- [ ] HTTP contract covered in API tests if API changed.
- [ ] No unnecessary duplicate tests across layers.

4. Confirm concurrency and performance

- [ ] New async code does not block the runtime.
- [ ] New shared state has an intentional synchronization strategy.
- [ ] New background tasks have cancellation/shutdown behavior.
- [ ] New parallelism is bounded.
- [ ] New DB/API calls have timeout/error behavior where relevant.
- [ ] No obvious hot-path allocation or clone problem was introduced.
- [ ] No unbounded channel/task growth was introduced.
- [ ] No lock guard is held across .await.

5. Confirm security/data safety

- [ ] No secrets/PII are logged.
- [ ] Ownership/auth checks exist where relevant.
- [ ] Internal errors are not leaked to users.
- [ ] User input cannot cause panic in normal request paths.

6. Confirm quality

- [ ] Code is simple enough.
- [ ] No speculative abstractions.
- [ ] No obvious Rust footguns.
- [ ] No security/data leakage issues.

7. Confirm commands

- [ ] cargo fmt --all -- --check
- [ ] cargo clippy --all-targets --all-features -- -D warnings
- [ ] cargo test --all-features

8. Final answer

- [ ] Summarize behavior added.
- [ ] Summarize tests.
- [ ] Summarize review findings fixed or remaining.
- [ ] Summarize commands run.
