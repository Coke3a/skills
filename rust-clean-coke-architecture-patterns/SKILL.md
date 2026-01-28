---
name: rust-clean-arch
description: Applies Rust Clean Architecture patterns (route->usecase->repository) with domain entities/value objects, ORM repository implementations, and usecase-centric error handling for backend and worker services. Use when scaffolding, refactoring, or reviewing Rust services that need route/usecase/repository layering, ORM data access, and explicit error handling.
---

# Rust Clean Architecture (route->usecase->repository)

## What this skill is for
Codifies a portable Rust Clean Architecture layout that separates HTTP routes, usecases, and repositories, with domain entities/value objects in a shared crate and infra adapters in a separate crate. It provides templates, error-handling rules, and workflows that mirror the stream-rokuo patterns while staying domain-agnostic.

## When to use (trigger phrases)
- "clean architecture" or "route->usecase->repository"
- "axum route" or "thin router"
- "usecase orchestration" or "application service"
- "repository trait" or "port/adapter"
- "worker job usecase" or "background processing"
- "diesel ORM" or "repository implementation"
- "error handling mapping" or "usecase errors"

## Quick start checklist
- [ ] Define or update the canonical layout (backend/, worker/, crates/domain, crates/infra, crates/observability).
- [ ] Create value objects + entities in crates/domain with invariant checks.
- [ ] Define repository traits (ports) in crates/domain.
- [ ] Implement a usecase with explicit input/output and guard clauses.
- [ ] Implement the ORM repository in crates/infra.
- [ ] Wire route -> usecase -> repository in backend and worker.
- [ ] Add usecase tests (mocks/fakes) and domain unit tests.
- [ ] Add tracing spans and structured logs; avoid secrets/PII.

## Decision defaults
- Default: async service with axum routes, usecase structs that depend on Arc<dyn Repo>, repository traits in crates/domain returning Option for not-found, Diesel + diesel_async in crates/infra, and usecase-centric error mapping.
- Escape hatch: if forced to use sync Diesel, wrap blocking repo calls in spawn_blocking or a dedicated thread pool.

## Links
- Architecture: ARCHITECTURE.md
- Style: STYLE.md
- Errors: ERROR_HANDLING.md
- ORM repositories: REPOSITORY_ORM.md
- Workflows: WORKFLOWS.md
- Examples: EXAMPLES.md
- Evaluations: EVALS.md
- Templates: templates/README.md
