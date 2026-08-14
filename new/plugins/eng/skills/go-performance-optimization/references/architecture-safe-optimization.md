# Architecture-Safe Optimization

Dependency direction:

```text
handlers -> usecases -> domain
infra -> domain interfaces
```

- Handlers stay thin; no business logic, no SQL.
- Usecases orchestrate and own user-facing semantics.
- Domain stays pure and owns entities, value objects, invariants, and repository interfaces.
- Infra owns IO, SQL, sqlc-generated code, pool access, and DB-specific performance optimization.
- Repository interfaces stay the boundary.
- DTOs must not leak into domain; sqlc-generated types must not leak past
  `internal/infra/postgres`.
- Do not move code into the wrong layer to shave function calls or allocations.
- Do not bypass repository interfaces ("just run the SQL in the handler for speed") unless the
  user explicitly approves an architecture change.
- Batch/aggregate methods added for N+1 fixes are still domain interface methods with intent
  names — the optimization lives in the infra implementation.
- Prefer optimizing inside the correct layer first.
- If optimization requires an architecture change, report it as a proposed design change before
  implementation.
