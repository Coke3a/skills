# Coding Style

## Naming conventions

| Item | Convention | Example |
|------|-----------|---------|
| Modules/files | snake_case | `item_postgres.rs`, `create_item.rs` |
| Types | CamelCase | `CreateItemUseCase`, `ItemRepository` |
| Usecase structs | `{Action}{Entity}UseCase` | `CreateItemUseCase` |
| Background usecases | `{Name}UseCase` | `CleanupUseCase` |
| Usecase I/O | `{Action}{Entity}Input/Output` | `CreateItemInput` |
| Request/Response DTOs | `{Action}{Entity}Request/Response` | `CreateItemRequest` |
| ID newtypes | `{Entity}Id(Uuid)` | `ItemId`, `OrderId` |
| Repository traits | `{Entity}Repository` | `ItemRepository` |
| Repository impls | `{Entity}Postgres` | `ItemPostgres` |
| Row structs | `{Entity}Row` / `New{Entity}Row` | `ItemRow`, `NewItemRow` |
| Value objects | `{Entity}{Field}` | `ItemName`, `WebhookUrl` |

## Entity style
- Private fields (no `pub`).
- `new()` creates fresh entities (generates IDs, sets timestamps, enforces invariants).
- `from_existing()` reconstructs from database (takes all fields, no validation).
- Getters return references: `&self -> &T` or `&self -> Option<&T>`.
- State transitions take `&mut self` and return `Result<(), DomainError>`.

## Value object style
- `new()` validates input, returns `Result<Self, DomainError>`.
- `from_trusted()` skips validation (for DB reconstruction only).
- `as_str()` returns `&str` for string-based value objects.
- ID newtypes: `new()` generates UUID, `from_uuid()` wraps existing, `as_uuid()` returns `&Uuid`.

## Usecase style
- Take `Arc<dyn Repo>` trait object dependencies (not generics).
- `new()` constructor stores dependencies.
- `execute()` or domain-specific method name (e.g., `sweep()`).
- Use explicit input/output structs for HTTP-facing usecases.
- Start with guard clauses for validation and existence checks.
- Prefer flat control flow and early returns over deep nesting.
- Use `?` operator with `From` impls for error conversion.
- Log: `info` for success, `warn` for business violations, `error` for failures.

## Handler style
- Handlers are async functions (not methods on structs).
- Extract `State(state): State<AppState>` and auth extractor as needed.
- Create repo implementations from `state.db_pool` inline.
- Instantiate usecase with repo dependencies.
- Map request DTO to usecase input, call usecase, return `Result<impl IntoResponse, ApiError>`.
- Request/Response DTOs defined in handler file with `#[derive(Deserialize)]` / `#[derive(Serialize)]`.

## Router organization style
One directory per domain under `handlers/routers/{domain}/`:
- `mod.rs` -- declares sub-modules, exports `pub fn router() -> Router<AppState>`.
- One file per handler action: `create.rs`, `list.rs`, `get.rs`, `update.rs`, `delete.rs`.

## Background task handler style
- Single `spawn()` function returning `JoinHandle<()>`.
- Takes `Arc<UseCase>`, `CancellationToken`, and config params.
- Uses `tokio::time::interval` + `tokio::select!` with `cancel.cancelled()`.
- Logs on start, on meaningful work, and on shutdown.

## Early return pattern

WHY it matters: deeply nested code is harder to read, harder to test, and invites bugs. Guard clauses at the top of a function reject invalid states immediately, keeping the happy path flat and left-aligned.

```rust
// GOOD: guard clauses, flat flow
pub async fn execute(&self, input: Input) -> Result<Output, UsecaseError> {
    if input.name.is_empty() {
        return Err(UsecaseError::Validation("name required".into()));
    }
    let entity = self.repo.find_by_id(input.id).await?
        .ok_or_else(|| UsecaseError::NotFound("entity not found".into()))?;
    // ... happy path continues flat
    Ok(output)
}

// BAD: deep nesting
pub async fn execute(&self, input: Input) -> Result<Output, UsecaseError> {
    if !input.name.is_empty() {
        if let Some(entity) = self.repo.find_by_id(input.id).await? {
            // ... indented further and further
        }
    }
}
```

## Import ordering
1. `std` / `core` / `alloc`
2. External crates (from `Cargo.toml`)
3. Workspace crates
4. `super::` / `crate::`

## Comment guidance
- Comment on the "why", not the "what" or "how".
- Use `///` rustdoc for public structs/enums and methods.
- Use `//!` for module-level docs.
- If a function needs long comments, refactor into smaller named functions.
- Turn TODOs into tracked issues; reference the issue in code.
