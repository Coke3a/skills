---
name: rust-clean-architecture
description: Guides Rust backend feature architecture using Axum, Diesel, and PostgreSQL with Clean Architecture layers (handlers → usecases → domain; infra → domain traits). Use when creating or refactoring handlers, usecases, repository traits, Diesel implementations, domain entities, value objects, DTOs, error types, or feature layout. Do not use for TDD workflow, CI/CD setup, code review, or performance profiling.
---

# Rust Clean Architecture

## Use this when

- Creating or refactoring a Rust backend feature with Axum, Diesel, and PostgreSQL.
- Scaffolding handlers, usecases, domain entities, value objects, or repository traits.
- Adding a Diesel repository implementation, route group, or external-service port.
- Deciding which layer owns a piece of code (handler, usecase, domain, infra).
- Designing the error flow from `DomainError` / `RepoError` to `ApiError`.
- Renaming files, modules, or types to match project conventions.

## Do not use this when

- Driving the red/green/refactor loop → use `coke-eng:rust-tdd-feature-workflow`.
- Setting up GitHub Actions or deployment → use `coke-eng:rust-ci-cd`.
- Producing a final review report → use `coke-eng:rust-code-review`.
- Profiling, benchmarking, or hot-path tuning → use `coke-eng:rust-performance-optimization`.

## Core rules

Dependency direction:

```text
handlers -> usecases -> domain
infra -> domain traits
```

- Handlers extract HTTP, define DTOs, instantiate infra, and call usecases. No business logic.
- Usecases own orchestration and user-facing error semantics. They depend on domain traits only.
- Domain owns entities, value objects, invariants, repository traits, service traits, and domain errors. It is framework-free.
- Infra implements repository/service traits with Diesel and external clients.
- Domain must not import Axum, Diesel, schema, DTOs, or infra types.
- DTOs must not reach domain. Diesel row structs must not leak past infra.
- Errors flow `DomainError → UsecaseError → ApiError` and `RepoError → UsecaseError → ApiError`.
- Use `thiserror` for error enums; reserve `anyhow` for infra/internal wrapping when the project already follows that pattern.
- Never `unwrap()` or `expect()` outside tests or code explicitly marked example-only.
- Every `mod.rs` is declaration-only (`pub mod ...;`) — no `pub use`, functions, route builders, tests, or wiring.

## Workflow

1. Pick the workflow file matching the change (see Load more detail).
2. Read only the references needed for the task.
3. Use templates in `templates/` as starting points; adapt names to the actual project.
4. Verify with the three cargo commands below before reporting done.

## Load more detail

- Layer responsibilities, file layout, handler/router organization, architecture checks → `references/architecture.md`
- Naming patterns and per-layer style → `references/coding-style.md`
- Error type responsibilities, conversion patterns, ApiError mapping table → `references/error-handling.md`
- Repository trait pattern and Diesel implementation pattern → `references/repository-orm.md`
- Idiomatic Rust patterns used in this architecture → `references/rust-idioms.md`
- Advanced patterns (transactions, services, complex flows) → `references/advanced-patterns.md`
- Testing conventions referenced by this architecture → `references/testing.md`
- Lint configuration assumed by these rules → `references/linting.md`
- Module/file documentation conventions → `references/documentation.md`

## Workflows

| Workflow                          | Use for                                                              |
| --------------------------------- | -------------------------------------------------------------------- |
| `workflows/scaffold-feature.md`   | New feature across domain, usecase, infra, and handler layers       |
| `workflows/add-router-domain.md`  | Adding a route group while preserving handler boundaries             |
| `workflows/refactor-to-layers.md` | Moving mixed handler/business/IO code into clean layers              |
| `workflows/add-background-task.md` | Adding a background task that preserves dependency direction        |

## Templates

| Template                        | Use for                                                                  |
| ------------------------------- | ------------------------------------------------------------------------ |
| `templates/domain_entity.rs`    | Entity with private fields, `new`, `from_existing`, getters, transitions |
| `templates/value_object.rs`     | ID newtype, validated value object, enum/state object                    |
| `templates/repo_trait.rs`       | Domain repository trait with `RepoError` return                          |
| `templates/repo_diesel_impl.rs` | Diesel repository implementation with centralized error mapping          |
| `templates/usecase.rs`          | Usecase input/output, orchestration, validation, repository call         |
| `templates/handler_axum.rs`     | Axum handler with DTO mapping and repo wiring                            |
| `templates/error_types.rs`      | Layered error enums and `From` conversions                               |
| `templates/background_job.rs`   | Background task scaffolding                                              |

## Related skills

- `coke-eng:rust-tdd-feature-workflow` — test design and the red/green/refactor loop.
- `coke-eng:rust-code-review` — final review against these rules.
- `coke-eng:rust-performance-optimization` — optimizing without breaking layer boundaries.
- `coke-eng:rust-ci-cd` — automation that ships the code produced under this skill.

## Definition of done

- Dependency direction is respected; domain has no Axum/Diesel imports; DTOs/rows stay in their owning layer.
- Names match `references/coding-style.md`.
- Error flow matches `references/error-handling.md`.
- Every `mod.rs` is declaration-only.
- These commands pass in the downstream Rust project:

```sh
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features
```
