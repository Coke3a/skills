# Repository Review

## Traits

- Traits live in domain.
- Persistence traits live in `src/domain/repositories/`; external IO traits live in
  `src/domain/services/`.
- Methods are named by behavior.
- Return types are consistent.
- `find_by_*` uses `Result<Option<T>, RepoError>`.
- Write operations are clear about created/updated/deleted semantics.
- Domain types are used instead of DTOs or row structs.
- Repository traits do not mention Diesel, schema, row structs, pools, or provider SDK types.

## Diesel Implementations

- Implementations live in infra.
- Diesel implementations live under `src/infra/db/repositories/`.
- Queries stay in infra.
- Row structs are private to infra.
- Row to entity mapping is correct.
- Entity reconstruction uses `from_existing()`.
- Created/updated/deleted timestamp semantics are correct.
- Soft delete behavior is consistent if used.
- Transactions are used where partial writes would be dangerous.
- Uniqueness conflicts are mapped correctly.
- Pool error mapping is centralized.
- Diesel error mapping is centralized.
- Raw SQL is avoided unless explicitly justified.

## Concurrency and Data Safety

- DB pool usage is safe under concurrent requests.
- Transaction scopes are short.
- Long-running DB operations are isolated or bounded.
- Concurrent writes have intended conflict/idempotency behavior where relevant.
- Tenant/user ownership is preserved in queries.
