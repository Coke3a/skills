# Evaluations

Use these prompts to check whether the skill produces focused Rust Clean Architecture output with generic examples.

## Eval 1: Scaffold generic feature

**Input:** "Create a new ExampleEntity feature with create and get actions."

**Expected:**

- Entity in `src/domain/entities/` with private fields, `new()`, `from_existing()`, and getters.
- ID newtype and validated value object in `src/domain/value_objects/`.
- Repository trait in `src/domain/repositories/`.
- Usecase with `Arc<dyn ExampleRepository>`, input/output structs, and no HTTP or Diesel imports.
- Diesel implementation in `src/infra/db/repositories/` with private row structs and centralized error mapping.
- Handler creates infra repository from `AppState`, instantiates usecase, maps DTOs, and returns `Result<impl IntoResponse, ApiError>`.

**Pass/fail checklist:**

- [ ] Dependency direction is `handlers -> usecases -> domain`
- [ ] Infra implements domain repository traits
- [ ] Domain imports no Axum, Diesel, schema, handler DTOs, or infra types
- [ ] Usecase imports no Axum, Diesel, schema, row structs, or handler DTOs
- [ ] Handler contains no business logic
- [ ] Repository trait is in domain
- [ ] Repository implementation is in infra
- [ ] Row structs are private to infra

## Eval 2: Refactor mixed handler

**Input:** "Refactor this Axum handler that validates input, queries Postgres, and builds a response into clean layers."

**Expected:**

- Request/response DTOs remain in the handler layer.
- Domain validation moves to value objects and entities.
- Orchestration moves to a usecase.
- Repository trait moves to domain.
- Diesel code moves to infra.
- Errors flow through `DomainError` or `RepoError` into `UsecaseError`, then `ApiError`.

**Pass/fail checklist:**

- [ ] Handler only extracts, wires, maps, calls, and responds
- [ ] Usecase owns orchestration and user-facing errors
- [ ] Domain owns invariants
- [ ] Infra owns Diesel query details
- [ ] Error conversion uses `From` and `?`

## Eval 3: Add Diesel repository

**Input:** "Add the Postgres repository implementation for ExampleRepository."

**Expected:**

- `ExamplePostgres` holds `Arc<PgPool>`.
- `ExampleEntityRow` uses `Queryable` and `Selectable`.
- `NewExampleEntityRow` uses `Insertable` and borrowed fields.
- Row conversion uses `ExampleEntity::from_existing()`.
- Queries use Diesel query builder only.
- Errors use `map_diesel_error()` and `map_pool_error()`.

**Pass/fail checklist:**

- [ ] No raw SQL is introduced
- [ ] Rows do not leave infra
- [ ] `find_by_id()` returns `Result<Option<ExampleEntity>, RepoError>`
- [ ] `update()` and `delete()` return `RepoError::NotFound` when no rows are affected
- [ ] Operation names follow `example_entity.operation`

## Eval 4: Error flow

**Input:** "Add error handling for validation failures, missing entities, unique constraint failures, and infrastructure failures."

**Expected:**

- `DomainError` represents validation and invariant failures.
- `RepoError` represents persistence and IO details.
- `UsecaseError` maps failures to user-facing semantics.
- `ApiError` maps usecase failures to HTTP status and response body.

**Pass/fail checklist:**

- [ ] `DomainError -> UsecaseError -> ApiError`
- [ ] `RepoError -> UsecaseError -> ApiError`
- [ ] Validation maps to HTTP 400
- [ ] Missing entity maps to HTTP 404
- [ ] Conflict maps to HTTP 409
- [ ] Internal failure maps to HTTP 500 with a generic client message

## Eval 5: Scope discipline

**Input:** "Use this skill to design the full test pyramid, CI pipeline, and code review process for a Rust backend."

**Expected:**

- The skill declines to own those workflows.
- The response says this skill only requires final downstream verification commands.
- The response directs the user to dedicated skills when available.

**Pass/fail checklist:**

- [ ] Does not define TDD steps
- [ ] Does not define CI/CD setup
- [ ] Does not define code review workflow
- [ ] Does not define complete testing strategy
- [ ] Keeps focus on architecture boundaries
