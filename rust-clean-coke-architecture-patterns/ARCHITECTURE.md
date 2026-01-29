# Architecture

## Findings summary (from stream-rokuo inspection)
- Usecases are structs with Arc dependencies and explicit input/output types; handlers are thin and mostly translate requests.
- Repository traits live in the domain layer and are implemented in infra with Diesel + a pooled Pg connection.
- Domain entities/value objects live in the domain layer, with value objects enforcing invariants at construction time.
- Errors are logged and enriched in lower layers, while usecases decide user-facing semantics and status codes.
- Observability uses tracing with structured fields and an optional error-notification layer.

## Canonical layout
Use this layout as the default baseline for any Rust project:

```
src/
  main.rs
  handlers/
  domain/
    entities/
    repositories/
    value_objects/
  usecases/
  infra/
    observability/
    payments/
    db/
```

## Responsibility boundaries

### Handlers (HTTP controllers, CLI entrypoints, job runners)
- Parse and validate request shape (types, UUID parsing, required fields).
- Map request DTOs to usecase input structs.
- Call the usecase; map UsecaseError -> HTTP response.
- No business logic beyond request parsing and response mapping.

### Usecases (application services)
- Own orchestration, authorization, validation, and transaction boundaries.
- Use guard clauses/early returns to keep flow readable.
- Convert repo errors into usecase error variants with clear semantics.
- Own observability (spans, structured logs, error severity).

### Repositories (ports + adapters)
- Port traits live in `src/domain/repositories/*` and define the repository interfaces.
- Adapters live in `src/infra/db/repositories/*` and contain ORM methods/queries.
- I/O only: no domain decisions, no HTTP logic.
- Return rich errors with operation context; usecase decides mapping.

### Domain (entities + value objects)
- Entities are immutable-ish; mutations go through methods that preserve invariants.
- Value objects validate invariants in constructors and return DomainError on invalid input.
- Domain logic is pure and free of HTTP/ORM concerns.

### Infra (adapters to the outside world)
- All external integrations live under `src/infra/*` (database, payments, observability, email, queues, etc.).
- Infra depends on domain abstractions and implements ports defined in `src/domain/repositories`.

### Dependency flow (must)
- Handlers -> usecases -> domain repository traits.
- Infra depends on domain abstractions; domain does not depend on infra or frameworks.
- Handlers must not call infra directly; only usecases do so via domain traits.

### Observability
- Use tracing spans around usecase entry points.
- Log with structured fields (ids, counts, op names).
- Avoid secrets/PII in logs; redact or hash where needed.
