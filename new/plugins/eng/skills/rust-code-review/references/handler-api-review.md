# Handler/API Review

- DTOs stay in handler/API layer.
- App composition stays in `handlers/app/` or the project equivalent.
- Cross-route handler utilities stay in `handlers/shared/` or the project equivalent.
- Endpoint groups live under `handlers/routers/{surface}/`, grouped by traffic boundary or API
  surface before feature files.
- `mod.rs` files are declaration-only and contain only `pub mod ...;`.
- Request validation happens at the appropriate boundary or domain/usecase layer.
- Response DTO shape is intentional and stable.
- HTTP status codes match behavior.
- `ApiError` mapping is user-facing and does not leak infra details.
- Route wiring matches the API contract.
- Auth extraction is present where required.
- Handler has minimal logic.
- Handler extracts state/auth/request.
- Handler constructs dependencies or uses the project dependency injection pattern.
- Handler maps request DTO to usecase input.
- Handler calls the usecase once for the main behavior.
- Handler maps usecase output to response DTO.
- Handler returns `Result<impl IntoResponse, ApiError>` or the project equivalent.
- App startup and route assembly do not contain route handler logic.
- Handler does not call Diesel directly.
- Handler does not enforce domain invariants.
- Handler does not duplicate usecase logic.
- Handler tests cover status and DTO shape when API contract changed.
- Handler does not perform blocking or CPU-heavy work directly on async runtime threads.
- Handler does not spawn unbounded per-request tasks.
- External IO has timeout behavior where relevant.
