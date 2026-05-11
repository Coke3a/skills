# Architecture-Safe Optimization

Dependency direction:

```text
handlers -> usecases -> domain
infra -> domain traits
```

- Handlers stay thin.
- Handlers do not contain business logic.
- Handlers do not contain Diesel queries.
- Usecases orchestrate and own user-facing semantics.
- Domain stays pure and owns entities, value objects, invariants, and repository traits.
- Infra owns IO, DB queries, Diesel rows, pool access, and DB-specific performance optimization.
- Repository traits stay the boundary.
- DTOs must not leak into domain.
- Diesel row structs must not leak into domain or handlers.
- Do not move code into the wrong layer to reduce function calls.
- Do not bypass repository traits unless the user explicitly approves an architecture change.
- Prefer optimizing inside the correct layer first.
- If optimization requires architecture change, report it as a proposed design change before implementation.
