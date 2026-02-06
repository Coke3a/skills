# Example feature overview

Feature: Endpoints (create endpoint with tier limit enforcement)

File placement:
- `src/domain/value_objects/ids/endpoint_id.rs` -> `EndpointId` newtype
- `src/domain/value_objects/validated/endpoint_name.rs` -> `EndpointName` validated string
- `src/domain/entities/endpoint.rs` -> `Endpoint` entity (private fields, new/from_existing)
- `src/domain/repositories/endpoint_repository.rs` -> `EndpointRepository` trait
- `src/infra/db/repositories/endpoint_postgres.rs` -> `EndpointPostgres` Diesel impl
- `src/usecases/endpoints/create_endpoint.rs` -> `CreateEndpointUseCase` + Input/Output
- `src/handlers/routers/endpoints/create.rs` -> Handler + Request/Response DTOs

Flow:
1) Handler extracts `AppState` and `AuthenticatedUser` from request.
2) Handler creates repo implementations from `state.db_pool`.
3) Handler instantiates usecase with repo dependencies.
4) Handler maps request DTO to usecase input struct.
5) Usecase validates inputs via domain value objects (DomainError auto-converts via `?`).
6) Usecase fetches subscription to determine tier limits.
7) Usecase calls `create_if_under_limit()` (atomic count + insert).
8) Usecase returns output struct or UsecaseError.
9) Handler maps output to response DTO, or ApiError handles error via `?`.

Error flow:
- DomainError (e.g., invalid name) -> UsecaseError::Validation via From impl
- RepoError (e.g., connection fail) -> UsecaseError::Infra via From impl
- UsecaseError -> ApiError via From impl -> HTTP response via IntoResponse
