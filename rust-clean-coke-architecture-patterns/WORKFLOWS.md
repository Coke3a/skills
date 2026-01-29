# Workflows

## Scaffold a new feature
1) Define domain entities and value objects
- [ ] Add value objects in `src/domain/value_objects` with validation in constructors.
- [ ] Add entity structs in `src/domain/entities` with invariant-preserving methods.

2) Define the repository port
- [ ] Add a trait in `src/domain/repositories/*`.
- [ ] Return `Option<T>` for not-found; use `Result<T, RepoError>` for failures.

3) Implement the usecase
- [ ] Create a usecase struct with `Arc<dyn Repo>` dependencies.
- [ ] Add explicit input/output structs.
- [ ] Use guard clauses for auth/validation/existence checks.
- [ ] Map repo errors into `UsecaseError`.

4) Implement the infra repository
- [ ] Add Diesel queries in `src/infra/db/repositories/*`.
- [ ] Map DB rows to domain entities via a small mapping layer.
- [ ] Enrich errors with operation context.

5) Wire handler -> usecase
- [ ] Add an axum handler file under `src/handlers`.
- [ ] Parse request DTOs and call the usecase.
- [ ] Map `UsecaseError` to HTTP status codes.

6) Add tests
- [ ] Domain unit tests for value objects and entity methods.
- [ ] Usecase tests with mocks/fakes for repository traits.

7) Observability
- [ ] Add `#[tracing::instrument]` to usecase entry points.
- [ ] Log structured fields; avoid secrets/PII.

8) Final review checklist
- [ ] Handlers contain no business logic.
- [ ] Usecases own error semantics.
- [ ] Repositories only do IO.
- [ ] DTOs are separate from domain entities.

## Refactor existing code into clean layers
1) Identify the entrypoint
- [ ] Locate the current handler or job runner and its business logic.

2) Extract a usecase
- [ ] Move orchestration into a usecase struct.
- [ ] Define input/output structs and error enum.

3) Define repository ports
- [ ] Identify data access in the handler and define traits in domain.

4) Move IO into infra
- [ ] Implement repo traits in infra with ORM queries.

5) Thin the route
- [ ] Reduce to request parsing + usecase invocation + error mapping.

6) Add tests
- [ ] Cover usecase behavior with mocks.

## Feedback loop pattern
- [ ] Run unit tests and fix failures.
- [ ] Run lint/clippy and fix warnings.
- [ ] Re-run tests.
- [ ] Final review for layering and error mapping.
