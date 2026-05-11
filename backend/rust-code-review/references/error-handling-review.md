# Error Handling Review

Expected flow:

- `DomainError -> UsecaseError -> ApiError`
- `RepoError -> UsecaseError -> ApiError`

Check:

- `DomainError` represents validation and business invariants only.
- `RepoError` represents persistence and IO details.
- `UsecaseError` owns user-facing semantics.
- `ApiError` owns HTTP mapping.
- Conversion direction is clear.
- Not found cases map intentionally, commonly `Ok(None)` from `find_by_*` repositories
  and a usecase-level not-found error when user-facing.
- Conflict cases preserve enough detail to return the right user-facing response.
- Validation errors are created in domain/usecase, not infra.
- Infra/internal errors do not leak directly into domain or handlers.
- Domain errors do not depend on HTTP status codes.
- `ApiError` does not expose Diesel-specific details.
- Error messages are useful but do not reveal sensitive details.
- No `unwrap()` or `expect()` appears in production without a clear safety reason.
- Background task errors are logged, propagated, or otherwise handled.
- Async task failures are not silently discarded when failure matters.
