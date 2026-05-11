# Refactor existing code into clean layers

1) Identify the entrypoint
- [ ] Locate the current handler and its inline business logic.

2) Extract a usecase
- [ ] Move orchestration into a usecase struct with `Arc<dyn Repo>` deps.
- [ ] Define input/output structs and use `From` impls for errors.

3) Define repository ports
- [ ] Identify data access and define traits in domain.

4) Move IO into infra
- [ ] Implement repo traits with Diesel queries, `Row`/`NewRow` structs, centralized error mapping.

5) Thin the handler
- [ ] Reduce to: create repos from state -> instantiate usecase -> parse input -> call usecase -> return response.

6) Add tests
- [ ] Cover usecase behavior with mock repos.

## Feedback loop pattern
- [ ] Run `cargo clippy` and fix warnings.
- [ ] Run `cargo test` and fix failures.
- [ ] Re-run tests.
- [ ] Final review for layering and error mapping.
