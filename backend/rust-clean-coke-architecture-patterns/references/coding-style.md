# Coding Style Reference

This reference covers only naming and style conventions needed for the Clean
Architecture pattern.

## Naming

| Item | Pattern | Example |
|---|---|---|
| Module/file | snake_case | `create_example_entity.rs` |
| Entity | PascalCase | `ExampleEntity` |
| ID newtype | `{Entity}Id` | `ExampleEntityId` |
| Validated value object | `{Entity}{Field}` | `ExampleEntityName` |
| Repository trait | `{Entity}Repository` | `ExampleRepository` |
| Repository impl | `{Entity}Postgres` | `ExamplePostgres` |
| Usecase | `{Action}{Entity}UseCase` | `CreateExampleEntityUseCase` |
| Input/output | `{Action}{Entity}Input/Output` | `CreateExampleEntityInput` / `CreateExampleEntityOutput` |
| Request/response DTO | `{Action}{Entity}Request/Response` | `CreateExampleEntityRequest` / `CreateExampleEntityResponse` |
| Row structs | `{Entity}Row` / `New{Entity}Row` | `ExampleEntityRow` / `NewExampleEntityRow` |
| Route handler | snake_case action | `create_example_entity` |
| Table | snake_case plural | `example_entities` |
| Generic columns | snake_case | `owner_id`, `column_text`, `column_url`, `status` |

## Entity style

- Use private fields.
- Use `new()` for fresh entity creation.
- Use `from_existing()` for database reconstruction.
- Use getters instead of public fields.
- Add state transition methods only when the entity owns a real invariant.
- Return `Result<(), DomainError>` from fallible transitions.
- Keep entities free of Axum, Diesel, schema, DTOs, and infra types.

## Value object style

- Use ID newtypes for domain IDs.
- Use validated value objects for user-provided fields that have invariants.
- Use `new()` for validation.
- Use `from_trusted()` only for database reconstruction or internally trusted values.
- Return `DomainError` for validation failures.

## Usecase style

- Inject repositories as `Arc<dyn RepositoryTrait>`.
- Define explicit input and output structs.
- Validate input by constructing domain value objects.
- Keep orchestration and user-facing error decisions in the usecase.
- Prefer guard clauses and `?` over nested control flow.
- Do not import Axum, Diesel, schema modules, row structs, or handler DTOs.

## Handler style

- Keep handlers as async functions.
- Extract state, auth/user context, path/query params, and JSON bodies.
- Define request and response DTOs in the handler layer.
- Instantiate repository implementations from `AppState`.
- Instantiate usecases in the handler.
- Map request DTOs to usecase input.
- Map usecase output to response DTOs.
- Return `Result<impl IntoResponse, ApiError>`.
- Do not put business logic in handlers.

## Infra repository style

- Name implementations `{Entity}Postgres`.
- Hold `Arc<PgPool>`.
- Define private `{Entity}Row` and `New{Entity}Row` structs.
- Use Diesel query builder only.
- Use centralized error mapping helpers.
- Return domain entities, not row structs.
