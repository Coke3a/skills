# Example feature overview

Feature: Projects (generic CRUD example)

File placement:
- `src/domain/entities/project.rs` -> `Project`, `ProjectId`
- `src/domain/value_objects/project.rs` -> `ProjectName`, `ProjectStatus`
- `src/domain/repositories/project.rs` -> `ProjectRepository` trait
- `src/infra/db/repositories/project.rs` -> Diesel impl
- `src/usecases/project.rs` -> `ProjectUseCase`
- `src/handlers/project.rs` -> handlers

Flow:
1) Route parses request DTOs.
2) Usecase validates inputs and orchestrates.
3) Repository performs DB IO.
4) Usecase maps errors to user-facing variants.
5) Route maps error variants to HTTP status codes.
