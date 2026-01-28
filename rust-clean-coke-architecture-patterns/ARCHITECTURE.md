# Architecture

## Findings summary (from stream-rokuo inspection)
- Backend and worker are separate binaries that both wire routes -> usecases -> repositories.
- Usecases are structs with Arc dependencies and explicit input/output types; routes are thin and mostly translate requests.
- Repository traits live in crates/domain and are implemented in crates/infra with Diesel + a pooled Pg connection.
- Domain entities/value objects live in crates/domain, with value objects enforcing invariants at construction time.
- Errors are logged and enriched in lower layers, while usecases decide user-facing semantics and status codes.
- Observability uses tracing with structured fields and an optional error-notification layer.

## Canonical layout
Use this layout as the default baseline for new services:

```
backend/
  src/
    axum_http/
      routers/
    usecases/
    config/
worker/
  src/
    axum_http/
      routers/
    usecases/
    config/
crates/
  domain/
    entities/
    value_objects/
    repositories/
  infra/
    db/
      postgres/
        schema.rs
        migrations/
      repositories/
    storages/
    web_driver/
  observability/
```

## Responsibility boundaries

### Routes (HTTP controllers)
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
- Port traits live in crates/domain.
- Adapters live in crates/infra and contain ORM details.
- I/O only: no domain decisions, no HTTP logic.
- Return rich errors with operation context; usecase decides mapping.

### Domain (entities + value objects)
- Entities are immutable-ish; mutations go through methods that preserve invariants.
- Value objects validate invariants in constructors and return DomainError on invalid input.
- Domain logic is pure and free of HTTP/ORM concerns.

### Observability
- Use tracing spans around usecase entry points.
- Log with structured fields (ids, counts, op names).
- Avoid secrets/PII in logs; redact or hash where needed.
