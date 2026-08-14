# Coding Style Reference

This reference covers only naming and style conventions needed for the Clean Architecture pattern.

## Naming

| Item                   | Pattern                            | Example                                                      |
| ---------------------- | ---------------------------------- | ------------------------------------------------------------ |
| Module/file            | snake_case                         | `create_example_entity.rs`                                   |
| Entity                 | PascalCase                         | `ExampleEntity`                                              |
| ID newtype             | `{Entity}Id`                       | `ExampleEntityId`                                            |
| Validated value object | `{Entity}{Field}`                  | `ExampleEntityName`                                          |
| Repository trait       | `{Entity}Repository`               | `ExampleRepository`                                          |
| Repository impl        | `{Entity}Postgres`                 | `ExamplePostgres`                                            |
| Usecase                | `{Action}{Entity}UseCase`          | `CreateExampleEntityUseCase`                                 |
| Input/output           | `{Action}{Entity}Input/Output`     | `CreateExampleEntityInput` / `CreateExampleEntityOutput`     |
| Request/response DTO   | `{Action}{Entity}Request/Response` | `CreateExampleEntityRequest` / `CreateExampleEntityResponse` |
| Row structs            | `{Entity}Row` / `New{Entity}Row`   | `ExampleEntityRow` / `NewExampleEntityRow`                   |
| Route handler          | snake_case action                  | `create_example_entity`                                      |
| Table                  | snake_case plural                  | `example_entities`                                           |
| Generic columns        | snake_case                         | `owner_id`, `column_text`, `column_url`, `status`            |

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
- Put ID newtypes in `domain/value_objects/ids/`.
- Put validated strings/fields in `domain/value_objects/validated/`.
- Put domain enums and state objects in `domain/value_objects/enums/`.
- Use `new()` for validation.
- Use `from_trusted()` only for database reconstruction or internally trusted values.
- Return `DomainError` for validation failures.

## Usecase style

- Put usecases in `usecases/{feature}/{action}.rs`.
- Keep one main usecase struct per leaf file.
- Inject repositories as `Arc<dyn RepositoryTrait>`.
- Inject external service ports as `Arc<dyn ServiceTrait>`.
- Define explicit input and output structs.
- Validate input by constructing domain value objects.
- Call concrete infra only through domain repository/service traits.
- Keep orchestration and user-facing error decisions in the usecase.
- Prefer guard clauses and `?` over nested control flow.
- Do not import Axum, Diesel, schema modules, row structs, or handler DTOs.

## Domain service style

- Put external-service traits in `domain/services/{example_service}.rs`.
- Put `ServiceError` in `domain/services/error.rs`.
- Use service traits for provider clients, auth clients, payment clients, notification dispatchers,
  webhook verifiers, and other external IO ports.
- Keep concrete HTTP/SDK clients in `infra/`.
- Do not put provider SDK types in entities, value objects, repository traits, or usecase DTOs.

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
- Keep `handlers/app/` for state, server startup, route assembly, middleware, and dispatch glue.
- Keep `handlers/shared/` for cross-route handler utilities such as auth extractors, API errors,
  and response helpers.
- Keep `handlers/routers/{surface}/` grouped by traffic boundary or API surface before feature
  files, such as `public_api`, `admin_api`, `webhook`, or `dashboard`.
- Keep `mod.rs` files declaration-only: only `pub mod ...;`; no `pub use`, functions, consts,
  route builders, tests, type aliases, or wiring logic.
- Keep route/action logic in leaf files, not in `mod.rs` or app startup.

## Infra repository style

- Name implementations `{Entity}Postgres`.
- Hold `Arc<PgPool>`.
- Define private `{Entity}Row` and `New{Entity}Row` structs.
- Use Diesel query builder only.
- Use centralized error mapping helpers.
- Return domain entities, not row structs.

## Module declaration style

- Keep every `mod.rs` declaration-only with only `pub mod ...;`.
- Do not put `pub use`, functions, consts, type aliases, route builders, tests, or wiring logic in
  `mod.rs`.
