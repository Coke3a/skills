---
name: rust-dev
description: >
  Complete Rust development guide combining clean architecture patterns
  (handlers -> usecases -> repositories -> domain) with idiomatic Rust
  best practices. Use when: (1) scaffolding new Rust features or
  handlers, (2) writing usecases with error handling and early returns,
  (3) implementing Diesel ORM repositories, (4) reviewing Rust code
  for borrowing, cloning, or ownership issues, (5) defining domain
  entities, value objects, or error types, (6) adding background tasks,
  (7) writing tests for usecases or domain logic, (8) optimizing Rust
  code for performance or idiomatic patterns, (9) any Rust web service
  work with Axum, Diesel, or clean architecture layers.
  Stack: Axum 0.8, Diesel 2.2, diesel-async 0.5, deadpool, thiserror,
  anyhow, tokio, tracing.
license: MIT
compatibility: Rust 1.70+, Cargo
metadata:
  author: coke
  version: "1.0.0"
  sources:
    - apollographql/rust-best-practices
    - rust-clean-coke-architecture-patterns
allowed-tools: >
  Bash(cargo:*) Bash(rustc:*) Bash(rustfmt:*) Bash(clippy:*)
  Read Write Edit Glob Grep
---

# Rust Development Guide

This skill combines clean architecture patterns with idiomatic Rust best practices.
Architecture defines the structure; best practices define how you write code within it.

## Stack

- **Web framework**: Axum 0.8
- **ORM**: Diesel 2.2 + diesel-async 0.5 (no raw SQL — always use Diesel query builder)
- **Connection pool**: deadpool (`PgPool` type alias)
- **Error types**: thiserror (for all error enums), anyhow (only for wrapping in `Infra` variants)
- **Async runtime**: tokio (full features)
- **Logging**: tracing + tracing-subscriber (JSON output)
- **Database**: Supabase PostgreSQL
- **Migrations**: SQL files in `migrations/` at project root (Supabase-managed, NOT Diesel CLI)

## When to Read References

Read the relevant reference file for deeper guidance. Read ALL needed references in a single turn.

| Task | Read |
|------|------|
| Scaffolding a feature | `references/architecture.md`, `workflows/scaffold-feature.md` |
| Writing a handler | `templates/handler_axum.rs` |
| Writing a usecase | `templates/usecase.rs`, `references/coding-style.md` |
| Implementing a repository | `templates/repo_diesel_impl.rs`, `references/repository-orm.md` |
| Error handling design | `references/error-handling.md`, `templates/error_types.rs` |
| Code review / refactoring | `references/rust-idioms.md`, `references/linting.md` |
| Performance optimization | `references/rust-idioms.md` (performance section) |
| Writing tests | `references/testing.md` |
| Generics vs trait objects | `references/advanced-patterns.md` |
| Adding a background task | `templates/background_job.rs`, `workflows/add-background-task.md` |
| Adding a route group | `workflows/add-router-domain.md` |
| Refactoring to layers | `workflows/refactor-to-layers.md` |

## Architecture Overview

```
project-root/
  migrations/                   # Supabase SQL migrations (NOT Diesel CLI)
  Cargo.toml
  diesel.toml                   # Diesel config (for schema.rs generation only)
  src/
    main.rs                     # Entry: load config -> create pool -> start app
    lib.rs                      # Declare modules
    config/
      mod.rs                    # Re-exports
      config_loader.rs          # Load from env vars
    domain/
      mod.rs                    # Re-exports DomainError
      error.rs                  # DomainError enum
      entities/                 # Domain entities (private fields, new/from_existing)
      repositories/             # Repository trait interfaces + RepoError
      value_objects/             # IDs, validated strings, state machine enums
        ids/                    # {Entity}Id(Uuid) newtypes
        validated/              # Validated string wrappers
        enums/                  # State machine enums
    usecases/
      mod.rs                    # Re-exports UsecaseError + all usecases
      error.rs                  # UsecaseError + From impls
      {feature}/                # Feature-grouped usecases
      background/               # Background task usecases
    handlers/
      mod.rs                    # Declares all handler modules
      app.rs                    # AppState, start(), middleware, router assembly
      extractors/               # Auth extractors
      routers/                  # HTTP route handlers
        mod.rs                  # Re-exports ApiError + router modules
        error_response.rs       # ApiError(UsecaseError) -> IntoResponse
        {feature}/              # One dir per domain, one file per action
      {background_task}/        # Background task spawners
    infra/
      mod.rs                    # Declares db + external service modules
      db/
        mod.rs                  # Re-exports
        postgres_connection.rs  # PgPool type alias, create_pool()
        schema.rs               # Diesel auto-generated (do not edit)
        repositories/           # Concrete Postgres implementations
          mod.rs                # Re-exports all impls
          error_mapping.rs      # map_diesel_error(), map_pool_error()
          {entity}_postgres.rs  # Diesel impl with Row/NewRow
      {external_service}/       # External API clients (optional)
```

### Dependency Flow (strict)

```
handlers -> usecases -> domain (repository traits, entities, value objects)
                          ^
                          |
                    infra implements domain traits
```

- Handlers MUST NOT call infra directly. Only through usecases via domain traits.
- Domain has NO framework dependencies (no Axum, no Diesel).
- Infra implements domain-defined traits.

## Decision Defaults

- Usecases use `Arc<dyn Repo>` trait objects (not generics) for dependency injection
- Repository implementations hold `Arc<PgPool>` (deadpool)
- Error mapping uses centralized `map_diesel_error()` and `map_pool_error()` helpers
- Repos created in handlers: `Arc::new(EntityPostgres::new(Arc::clone(&state.db_pool)))`
- Background tasks: handler spawns loop with `CancellationToken`, usecase has logic, repo does DB
- Migrations: Supabase-managed SQL in `migrations/` at project root. Diesel is used ONLY as ORM for queries, not for migration management.
- Router organization: each domain gets `routers/{domain}/mod.rs` with `pub fn router() -> Router<AppState>`

## Error Handling

Errors flow through a layered chain. Repositories return detailed errors. Usecases own the decision of how to handle them.

### Error Types

| Type | Location | Purpose |
|------|----------|---------|
| `DomainError` | `domain/error.rs` | Validation and business rule violations |
| `RepoError` | `domain/repositories/error.rs` | Database/IO failures with rich context |
| `UsecaseError` | `usecases/error.rs` | User-facing error semantics |
| `ApiError` | `handlers/routers/error_response.rs` | HTTP response mapping (thin wrapper) |

### Conversion Chain

```
DomainError ──From──> UsecaseError ──From──> ApiError (IntoResponse)
RepoError   ──From──> UsecaseError ──From──> ApiError (IntoResponse)
```

Key: use `?` operator everywhere. `From` impls handle conversion automatically.

### HTTP Status Mapping

| UsecaseError | Status | Error Code |
|---|---|---|
| NotFound | 404 | NOT_FOUND |
| Validation | 400 | VALIDATION_ERROR |
| Conflict | 409 | CONFLICT |
| TierLimitExceeded | 409 | LIMIT_REACHED |
| RateLimited | 429 | RATE_LIMITED (+ headers) |
| Gone | 410 | GONE |
| Infra | 500 | INTERNAL_ERROR (generic msg, full chain logged) |

### Error Handling Rules

1. Use `thiserror` for ALL error enums. Use `anyhow` ONLY for wrapping in `RepoError::Db` and `UsecaseError::Infra`.
2. Never use `unwrap()` or `expect()` outside tests.
3. Prefer `?` operator over match chains for error propagation.
4. Repositories return detailed, specific error variants. Usecases decide what they mean.
5. `UsecaseError::Infra` logs full error chain server-side but returns generic message to client.

## Coding Style

### Naming Conventions

| Item | Pattern | Example |
|------|---------|---------|
| Module/file | snake_case | `create_endpoint.rs` |
| Usecase struct | `{Action}{Entity}UseCase` | `CreateEndpointUseCase` |
| Usecase I/O | `{Action}{Entity}Input/Output` | `CreateEndpointInput` |
| Handler DTOs | `{Action}{Entity}Request/Response` | `CreateEndpointRequest` |
| ID newtype | `{Entity}Id(Uuid)` | `EndpointId` |
| Repository trait | `{Entity}Repository` | `EndpointRepository` |
| Repository impl | `{Entity}Postgres` | `EndpointPostgres` |
| Row structs | `{Entity}Row` / `New{Entity}Row` | `EndpointRow` |
| Validated VO | `{Entity}{Field}` | `EndpointName` |
| Background usecase | `{Name}UseCase` | `DeliveryTimeoutUseCase` |

### Rust Idioms (Key Rules)

- Prefer `&T` over `.clone()` unless ownership transfer is required
- Use `&str` over `String`, `&[T]` over `Vec<T>` in function parameters
- Small `Copy` types (<=24 bytes) can be passed by value
- Use `Cow<'_, T>` when ownership is ambiguous
- Prefer iterators over manual for loops; avoid intermediate `.collect()` calls
- Never allocate early: pass `&str` and only `.to_string()` at the point of storage

### Entity Pattern

- Private fields (no `pub`)
- `new()`: creates fresh entity (generates ID, sets timestamps, enforces invariants)
- `from_existing()`: reconstructs from DB (all fields, no validation)
- Getters return references (`&self -> &T` or `Option<&T>`)
- State transitions: `&mut self` methods returning `Result<(), DomainError>`

### Usecase Pattern (Early Returns)

Usecases are the primary orchestration layer. They own error semantics and use the early return pattern for readability:

```rust
pub async fn execute(&self, input: Input) -> Result<Output, UsecaseError> {
    // Guard clause 1: validate input (early return on failure)
    let name = EntityName::new(input.name)?;  // DomainError -> UsecaseError via From

    // Guard clause 2: check existence (early return if not found)
    let entity = self.repo.find_by_id(&input.id).await?
        .ok_or_else(|| UsecaseError::NotFound("Entity not found".into()))?;

    // Guard clause 3: business rule check
    if !entity.is_active() {
        return Err(UsecaseError::Validation("Entity is not active".into()));
    }

    // Happy path: all guards passed
    // ... perform the operation ...

    Ok(output)
}
```

The goal: each guard clause validates one condition and returns early on failure. The happy path reads top-to-bottom without nesting.

### Handler Pattern

Handlers are thin. They parse requests, wire dependencies, call usecases, and map responses:

```rust
pub async fn create_entity(
    State(state): State<AppState>,
    auth: AuthenticatedUser,
    Json(body): Json<CreateEntityRequest>,
) -> Result<impl IntoResponse, ApiError> {
    let repo: Arc<dyn EntityRepository> =
        Arc::new(EntityPostgres::new(Arc::clone(&state.db_pool)));
    let usecase = CreateEntityUseCase::new(repo);
    let output = usecase.execute(CreateEntityInput { /* ... */ }).await?;
    Ok((StatusCode::CREATED, Json(CreateEntityResponse::from(output))))
}
```

## Repository Quick Reference

Repositories use Diesel ORM exclusively. No raw SQL.

### Row Conversion Pattern

- `EntityRow` (Queryable, Selectable): has `into_entity()` method
- `NewEntityRow<'a>` (Insertable): has `from_entity(&'a Entity)` with borrowed fields

### Error Mapping

```rust
let mut conn = self.pool.get().await.map_err(map_pool_error)?;
// Query:
.await.map_err(|e| map_diesel_error("entity.find_by_id", e))?;
```

Operation names: `"entity.operation"` (e.g., `"endpoint.create"`, `"user.find_by_id"`)

### Find Pattern (returns Option)

```rust
let result = table::table
    .find(id.as_uuid())
    .first::<EntityRow>(&mut conn)
    .await
    .optional()
    .map_err(|e| map_diesel_error("entity.find_by_id", e))?;
Ok(result.map(|row| row.into_entity()))
```

### Update with Not-Found Check

```rust
let rows_affected = diesel::update(table::table.find(id))
    .set(( /* fields */ ))
    .execute(&mut conn)
    .await
    .map_err(|e| map_diesel_error("entity.update", e))?;
if rows_affected == 0 {
    return Err(RepoError::NotFound(format!("Entity {} not found", id)));
}
```

## Background Tasks

Pattern: handler `spawn()` -> `JoinHandle<()>`, usecase sweep method, `CancellationToken`.

```rust
// In handler spawner:
pub fn spawn(usecase: Arc<MyUseCase>, cancel: CancellationToken, interval_secs: u64) -> JoinHandle<()> {
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_secs(interval_secs));
        loop {
            tokio::select! {
                _ = interval.tick() => { usecase.sweep().await; },
                _ = cancel.cancelled() => { break; },
            }
        }
    })
}
```

## Linting

Run regularly: `cargo clippy --all-targets --all-features -- -D warnings`

Key lints: `redundant_clone`, `large_enum_variant`, `needless_collect`.
Use `#[expect(clippy::lint)]` over `#[allow(...)]` with justification comment.

## Testing

- Descriptive names: `process_should_return_error_when_input_empty()`
- One assertion per test when possible
- Usecases: test with mock repository implementations
- Domain: unit tests on value objects and entity state transitions
- Use doc tests (`///`) for public API examples

## Documentation

- `//` comments explain *why* (safety, workarounds, design rationale)
- `///` doc comments explain *what* and *how* for public APIs
- Every `TODO` needs a linked issue: `// TODO(#42): ...`

## Templates

| Template | Target Location |
|----------|----------------|
| `templates/domain_entity.rs` | `src/domain/entities/*` |
| `templates/value_object.rs` | `src/domain/value_objects/` |
| `templates/repo_trait.rs` | `src/domain/repositories/*` |
| `templates/repo_diesel_impl.rs` | `src/infra/db/repositories/*` |
| `templates/usecase.rs` | `src/usecases/{feature}/*` |
| `templates/handler_axum.rs` | `src/handlers/routers/{feature}/*` |
| `templates/error_types.rs` | Error types across all layers |
| `templates/background_job.rs` | `src/usecases/background/*` + `src/handlers/{task}/` |

## Workflows

| Workflow | When to Use |
|----------|-------------|
| `workflows/scaffold-feature.md` | Adding a complete new feature end-to-end |
| `workflows/add-router-domain.md` | Adding a new route group under `/api/v1/` |
| `workflows/add-background-task.md` | Adding a new background sweeper/cleanup task |
| `workflows/refactor-to-layers.md` | Extracting clean layers from monolith code |

## References

| Reference | Topic |
|-----------|-------|
| `references/architecture.md` | Full layout, startup flow, middleware, auth extractors |
| `references/error-handling.md` | Complete error hierarchy with all From impls |
| `references/repository-orm.md` | Diesel patterns, transactions, upserts |
| `references/coding-style.md` | Naming, entity/usecase/handler conventions |
| `references/rust-idioms.md` | Borrowing, iterators, performance optimization |
| `references/testing.md` | Test patterns, snapshot testing, mock repos |
| `references/advanced-patterns.md` | Generics, dispatch, type state, pointers |
| `references/documentation.md` | Comments, doc comments, rustdoc |
| `references/linting.md` | Clippy configuration, workspace lints |
