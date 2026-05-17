# Clean Architecture Review

Use this reference to check whether changes followed `rust-clean-coke-architecture- patterns`.

## Dependency Direction

- Handlers call usecases.
- Usecases depend on domain types, domain repository traits, and domain service traits.
- Domain owns entities, value objects, invariants, errors, repository traits, and external-service
  traits.
- Infra implements domain repository/service traits.
- Infra owns Diesel rows, query builder code, pool access, and IO error mapping.

## Layer Ownership

- Handlers map request DTOs to usecase input, call usecases, map output to response DTOs, and map
  errors to API responses.
- Handler app composition, shared utilities, and endpoint routers stay separated.
- Endpoint router groups are named by traffic boundary or API surface before feature/action files.
- `mod.rs` files are declaration-only and contain only `pub mod ...;`.
- Usecases are grouped by feature/domain with action leaf files such as
  `usecases/example_feature/create.rs`.
- Usecases own orchestration, dependency calls, and user-facing error semantics.
- Domain owns invariants, business rules, entities, value objects, repository ports, and service
  ports.
- Infra owns persistence details and external IO adapters.

## Common Violations

- Handler contains business logic, domain invariants, or Diesel queries.
- Usecase imports Axum, Diesel, schema, row structs, or HTTP DTOs.
- Usecase directly constructs concrete HTTP/SDK clients instead of using a domain service trait.
- Domain imports handlers, infra, Axum, Diesel, schema, rows, or DTOs.
- Infra defines business semantics or user-facing HTTP behavior.
- Repository returns DTOs or row structs.
- Diesel rows leak outside infra.
- DTOs leak into domain.
- Entity exposes public mutable fields or relies on handlers/usecases to enforce core invariants.
- Value objects are skipped for user-provided fields with real validation rules.

## Usecase Organization

- Usecases live under `src/usecases/{feature}/{action}.rs`.
- Each action leaf file owns one main usecase struct plus its input/output structs.
- Usecases inject repositories as `Arc<dyn RepositoryTrait>` and service ports as
  `Arc<dyn ServiceTrait>`.
- Usecases call infra only through domain repository/service traits.

## Domain Model Organization

- Entities live in `src/domain/entities/`, use private fields, and expose constructors, getters,
  `from_existing()`, and explicit state transition methods.
- Value objects live under `src/domain/value_objects/ids/`, `validated/`, and `enums/` when those
  categories exist.
- External IO ports live in `src/domain/services/` and return `ServiceError`.

## Repository Pattern

- Repository traits live in `src/domain/repositories/`.
- Repository implementations live in `src/infra/db/repositories/`.
- `find_by_*` methods return `Result<Option<T>, RepoError>`.
- Diesel query builder is used unless the architecture skill explicitly allows an exception.
- Row to domain reconstruction uses `from_existing()`.
- Domain to row conversion borrows fields where practical.
- Row and NewRow structs stay private to infra.
- Pool and Diesel errors are mapped through centralized helpers.
