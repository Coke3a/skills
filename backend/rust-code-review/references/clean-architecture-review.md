# Clean Architecture Review

Use this reference to check whether changes followed `rust-clean-coke-architecture-
patterns`.

## Dependency Direction

- Handlers call usecases.
- Usecases depend on domain types and domain repository traits.
- Domain owns entities, value objects, invariants, errors, and repository traits.
- Infra implements domain repository traits.
- Infra owns Diesel rows, query builder code, pool access, and IO error mapping.

## Layer Ownership

- Handlers map request DTOs to usecase input, call usecases, map output to response
  DTOs, and map errors to API responses.
- Usecases own orchestration and user-facing error semantics.
- Domain owns invariants and business rules.
- Infra owns persistence details and external IO adapters.

## Common Violations

- Handler contains business logic, domain invariants, or Diesel queries.
- Usecase imports Axum, Diesel, schema, row structs, or HTTP DTOs.
- Domain imports handlers, infra, Axum, Diesel, schema, rows, or DTOs.
- Infra defines business semantics or user-facing HTTP behavior.
- Repository returns DTOs or row structs.
- Diesel rows leak outside infra.
- DTOs leak into domain.

## Repository Pattern

- Repository traits live in `src/domain/repositories/`.
- Repository implementations live in `src/infra/db/repositories/`.
- `find_by_*` methods return `Result<Option<T>, RepoError>`.
- Diesel query builder is used unless the architecture skill explicitly allows an
  exception.
- Row to domain reconstruction uses `from_existing()`.
- Domain to row conversion borrows fields where practical.
- Row and NewRow structs stay private to infra.
- Pool and Diesel errors are mapped through centralized helpers.
