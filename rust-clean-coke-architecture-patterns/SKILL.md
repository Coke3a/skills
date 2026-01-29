---
name: rust-clean-coke-architecture-patterns
description: Applies Rust Clean Architecture patterns (handlers->usecases->repositories) with domain entities/value objects, ORM repository implementations, and usecase-centric error handling for any Rust project. Use when scaffolding, refactoring, or reviewing Rust services that need clear layering, ORM data access, and explicit error handling.
---

# Rust Clean Architecture (route->usecase->repository)

## What this skill is for
Codifies a portable Rust Clean Architecture layout that separates handlers, usecases, and repositories under a single `src/` tree, with a pure domain core and infra adapters for external systems. It provides templates, error-handling rules, and workflows that mirror clean architecture patterns while staying domain-agnostic.

## When to use (trigger phrases)
- "clean architecture" or "route->usecase->repository"
- "axum route" or "thin router"
- "usecase orchestration" or "application service"
- "repository trait" or "port/adapter"
- "background job usecase" or "background processing"
- "diesel ORM" or "repository implementation"
- "error handling mapping" or "usecase errors"

## Quick start checklist
- [ ] Define or update the canonical layout (`src/handlers`, `src/usecases`, `src/domain`, `src/infra`).
- [ ] Create value objects + entities in `src/domain` with invariant checks.
- [ ] Define repository traits (ports) in `src/domain/repositories/*`.
- [ ] Implement a usecase with explicit input/output and guard clauses.
- [ ] Implement repository adapters in `src/infra/db/repositories/*` (ORM methods live here).
- [ ] Wire handler -> usecase -> repository via domain traits.
- [ ] Add usecase tests (mocks/fakes) and domain unit tests.
- [ ] Add tracing spans and structured logs; avoid secrets/PII.

## Default layout (must)
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

## Decision defaults
- Default: async service with axum handlers, usecase structs that depend on `Arc<dyn Repo>`, repository traits in `src/domain` returning `Option` for not-found, Diesel + diesel_async in `src/infra`, and usecase-centric error mapping.
- Escape hatch: if forced to use sync Diesel, wrap blocking repo calls in spawn_blocking or a dedicated thread pool.

## Dependency flow (must)
- Handlers -> usecases -> domain repository traits.
- Infra implements domain traits and holds all external integrations (`src/infra/*`).
- Domain has no dependencies on infra or frameworks.

## Links
- Architecture: ARCHITECTURE.md
- Style: STYLE.md
- Errors: ERROR_HANDLING.md
- ORM repositories: REPOSITORY_ORM.md
- Workflows: WORKFLOWS.md
- Examples: EXAMPLES.md
- Evaluations: EVALS.md
- Templates: templates/README.md
