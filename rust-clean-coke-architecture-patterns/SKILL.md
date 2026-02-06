---
name: rust-clean-coke-architecture-patterns
description: Applies Rust Clean Architecture patterns (handlers->usecases->repositories) with domain entities/value objects, ORM repository implementations, and usecase-centric error handling for any Rust project. Use when scaffolding, refactoring, or reviewing Rust services that need clear layering, ORM data access, and explicit error handling.
---

# Rust Clean Architecture (handler->usecase->repository)

## What this skill is for
Codifies the Rust Clean Architecture layout used in this codebase. Handlers are thin HTTP controllers, usecases orchestrate business logic via `Arc<dyn Trait>` dependencies, and repositories implement domain traits with Diesel async PostgreSQL. Templates and examples use real patterns from the codebase.

## When to use (trigger phrases)
- "clean architecture" or "handler->usecase->repository"
- "axum handler" or "thin controller"
- "usecase orchestration" or "application service"
- "repository trait" or "port/adapter"
- "background task" or "sweeper usecase"
- "diesel ORM" or "repository implementation"
- "error handling mapping" or "usecase errors"

## Quick start checklist
- [ ] Define or update the canonical layout (`src/handlers`, `src/usecases`, `src/domain`, `src/infra`, `src/config`).
- [ ] Create value objects in `src/domain/value_objects` (IDs, validated strings, state machine enums).
- [ ] Create entities in `src/domain/entities` with `new()` and `from_existing()` constructors, private fields, getters.
- [ ] Define repository traits in `src/domain/repositories/*`.
- [ ] Implement usecases with `Arc<dyn Repo>` dependencies and explicit input/output structs.
- [ ] Implement repository adapters in `src/infra/db/repositories/*` with `Row`/`NewRow` structs and centralized error mapping.
- [ ] Wire handler -> usecase -> repository. Repos are created in handlers from `AppState.db_pool`.
- [ ] Add usecase tests with mocks and domain unit tests.
- [ ] Add tracing spans and structured logs; avoid secrets/PII.

## Actual layout
```
src/
  main.rs                       # Entry point: loads config, creates pool, calls handlers::app::start()
  lib.rs                        # Declares top-level modules: config, domain, infra, handlers, usecases
  config/
    mod.rs
    config_model.rs             # DotEnvyConfig, Database, Server, Supabase, Cors, BackgroundTasks
    config_loader.rs            # load() function, env_or() helper
  domain/
    mod.rs                      # Re-exports DomainError
    error.rs                    # DomainError enum
    entities/
      mod.rs
      endpoint.rs               # new(), from_existing(), private fields, getters, state methods
      delivery.rs
      forwarding_session.rs
      event.rs
      destination.rs
      subscription.rs
      rate_limit.rs
      replay_quota.rs
      provider_preset.rs
    repositories/
      mod.rs                    # Re-exports all traits + RepoError
      error.rs                  # RepoError enum
      endpoint_repository.rs    # EndpointRepository trait
      ...                       # One trait per entity
    value_objects/
      mod.rs                    # Re-exports all value objects
      enums/
        session_status.rs       # State machine with transition_to(), is_terminal(), FromStr
        delivery_status.rs
        subscription_tier.rs    # Tier limits: max_endpoints(), rate_limit_per_hour(), etc.
        subscription_status.rs
        billing_cycle.rs
        http_method.rs
        attempt_type.rs
        connection_quality.rs
      ids/
        endpoint_id.rs          # EndpointId(Uuid) newtype: new(), from_uuid(), as_uuid(), Display
        session_id.rs
        delivery_id.rs
        ...                     # One per entity
      validated/
        endpoint_name.rs        # new() validates, from_trusted() skips validation, as_str()
        destination_url.rs
        webhook_url.rs
        destination_label.rs
  usecases/
    mod.rs                      # Re-exports all usecases, inputs, outputs, UsecaseError
    error.rs                    # UsecaseError + From<DomainError> + From<RepoError>
    endpoints/
      create_endpoint.rs        # CreateEndpointUseCase with input/output structs
    ingestion/
      receive_webhook.rs        # ReceiveWebhookUseCase
    background/
      delivery_timeout.rs       # DeliveryTimeoutUseCase
      heartbeat_sweeper.rs      # HeartbeatSweeperUseCase
      session_inactivity.rs     # SessionInactivityUseCase
      rate_limit_cleanup.rs     # RateLimitCleanupUseCase
  handlers/
    mod.rs
    app.rs                      # AppState, start(), spawn_background_tasks(), middleware stack
    extractors/
      auth.rs                   # AuthenticatedUser extractor (Supabase JWT)
    routers/
      mod.rs                    # Re-exports ApiError
      error_response.rs         # ApiError(UsecaseError) implementing IntoResponse
      default_routers.rs        # health_check, ready_check, not_found
      endpoints/
        create.rs               # Handler: creates repos from state, calls usecase
      ingestion/
        receive.rs              # Webhook ingress handler
    delivery_timeout/mod.rs     # spawn() function for background task loop
    heartbeat/mod.rs
    session_inactivity/mod.rs
    rate_limit/mod.rs
  infra/
    mod.rs
    db/
      mod.rs
      postgres_connection.rs    # PgPool type alias (deadpool), create_pool()
      schema.rs                 # Diesel auto-generated (do not edit)
      repositories/
        mod.rs                  # Re-exports all Postgres impls
        error_mapping.rs        # map_diesel_error(), map_pool_error()
        endpoint_postgres.rs    # EndpointRow, NewEndpointRow, EndpointPostgres
        ...                     # One impl per entity
```

## Decision defaults
- Usecases use `Arc<dyn Repo>` trait objects (not generics).
- Repository implementations hold `Arc<PgPool>` (deadpool).
- Error mapping uses centralized `map_diesel_error()` and `map_pool_error()` helpers.
- Repos create from handlers: `Arc::new(EntityPostgres::new(Arc::clone(&state.db_pool)))`.
- Background tasks: handler spawns loop, usecase has logic, repo does DB query.

## Dependency flow (must)
- Handlers -> usecases -> domain repository traits.
- Infra implements domain traits; domain has no framework dependencies.
- Handlers must not call infra directly; only through usecases via domain traits.

## Links
- Architecture: ARCHITECTURE.md
- Style: STYLE.md
- Errors: ERROR_HANDLING.md
- ORM repositories: REPOSITORY_ORM.md
- Workflows: WORKFLOWS.md
- Examples: EXAMPLES.md
- Evaluations: EVALS.md
- Templates: templates/README.md
