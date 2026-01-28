# Example feature overview

Feature: Projects (generic CRUD example)

File placement:
- `crates/domain/entities/project.rs` -> `Project`, `ProjectId`
- `crates/domain/value_objects/project.rs` -> `ProjectName`, `ProjectStatus`
- `crates/domain/repositories/project.rs` -> `ProjectRepository` trait
- `crates/infra/db/repositories/project.rs` -> Diesel impl
- `backend/src/usecases/project.rs` -> `ProjectUseCase`
- `backend/src/axum_http/routers/project.rs` -> routes

Flow:
1) Route parses request DTOs.
2) Usecase validates inputs and orchestrates.
3) Repository performs DB IO.
4) Usecase maps errors to user-facing variants.
5) Route maps error variants to HTTP status codes.
