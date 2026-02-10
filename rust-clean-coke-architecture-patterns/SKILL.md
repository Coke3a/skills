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

## Application startup flow

1. **`main.rs`**: `dotenvy::dotenv()` -> `config_loader::load()` -> `postgres_connection::create_pool()` -> `handlers::app::start(Arc<config>, Arc<pool>)`
2. **`app::start()`**: Creates `WsRegistry` -> Builds `AppState{config, db_pool, ws_registry}` -> `spawn_background_tasks()` -> builds middleware stack -> builds router with `http_api_routes()`, `ws_routes()`, `http_ingest_routes()`, `playground_routes()` -> `TcpListener::bind()` -> `axum::serve().with_graceful_shutdown()`
3. **Graceful shutdown**: `shutdown_signal()` waits for SIGTERM/Ctrl+C -> server stops accepting -> `cancel.cancel()` -> await all `JoinHandle`s

## Complete layout
```
src/
  main.rs                             # Entry point: loads config, creates pool, calls handlers::app::start()
  lib.rs                              # Declares top-level modules: config, domain, infra, handlers, usecases
  config/
    mod.rs                            # Re-exports config_model and config_loader
    config_model.rs                   # DotEnvyConfig, Database, Server, Supabase, Cors, BackgroundTasks, WebSocket
    config_loader.rs                  # load() function, env_or() helper for optional env vars with defaults
  domain/
    mod.rs                            # Re-exports DomainError
    error.rs                          # DomainError enum (InvalidField, BusinessRuleViolation, NotFound, Conflict, etc.)
    entities/
      mod.rs                          # Re-exports all entities
      endpoint.rs                     # Endpoint entity: new(), from_existing(), private fields, getters
      destination.rs                  # Destination entity
      event.rs                        # Event entity
      delivery.rs                     # Delivery entity
      subscription.rs                 # Subscription entity
      playground_session.rs           # PlaygroundSession entity
      rate_limit.rs                   # RateLimit entity
      replay_quota.rs                 # ReplayQuota entity
      provider_preset.rs              # ProviderPreset entity
    repositories/
      mod.rs                          # Re-exports all traits + RepoError
      error.rs                        # RepoError enum (Db, DbWithEntity, NotFound, UniqueViolation, etc.)
      endpoint_repository.rs          # EndpointRepository trait
      destination_repository.rs       # DestinationRepository trait
      event_repository.rs             # EventRepository trait + EventCursor, EventFilters, EventWithDeliverySummary
      delivery_repository.rs          # DeliveryRepository trait
      subscription_repository.rs      # SubscriptionRepository trait
      playground_session_repository.rs # PlaygroundSessionRepository trait
      rate_limit_repository.rs        # RateLimitRepository trait
      replay_quota_repository.rs      # ReplayQuotaRepository trait
      provider_preset_repository.rs   # ProviderPresetRepository trait
    value_objects/
      mod.rs                          # Re-exports ids/*, validated/*, enums/*
      enums/
        mod.rs                        # Re-exports all enums
        delivery_status.rs            # DeliveryStatus: Pending->InProgress->Success/Failed/Timeout
        playground_status.rs          # PlaygroundStatus state machine
        subscription_tier.rs          # SubscriptionTier with tier limits: max_endpoints(), rate_limit_per_hour()
        subscription_status.rs        # SubscriptionStatus enum
        billing_cycle.rs              # BillingCycle enum
        http_method.rs                # HttpMethod enum
        attempt_type.rs               # AttemptType enum
        connection_quality.rs         # ConnectionQuality enum
      ids/
        mod.rs                        # Re-exports all ID newtypes
        endpoint_id.rs                # EndpointId(Uuid): new(), from_uuid(), as_uuid(), Display
        destination_id.rs             # DestinationId(Uuid)
        event_id.rs                   # EventId(Uuid)
        delivery_id.rs                # DeliveryId(Uuid)
        subscription_id.rs            # SubscriptionId(Uuid)
        playground_session_id.rs      # PlaygroundSessionId(Uuid)
        rate_limit_id.rs              # RateLimitId(Uuid)
        replay_quota_id.rs            # ReplayQuotaId(Uuid)
        provider_preset_id.rs         # ProviderPresetId(Uuid)
        connection_id.rs              # ConnectionId(Uuid) - for WebSocket connections
      validated/
        mod.rs                        # Re-exports all validated strings
        endpoint_name.rs              # EndpointName: new() validates, from_trusted() skips, as_str()
        destination_url.rs            # DestinationUrl: validated URL
        webhook_url.rs                # WebhookUrl: validated webhook URL
        destination_label.rs          # DestinationLabel: validated label
  usecases/
    mod.rs                            # Re-exports all usecases, inputs, outputs, UsecaseError
    error.rs                          # UsecaseError + From<DomainError> + From<RepoError>
    endpoints/
      mod.rs                          # Re-exports endpoint usecases
      create_endpoint.rs              # CreateEndpointUseCase (Input/Output)
      list_endpoints.rs               # ListEndpointsUseCase (Input/Output)
      get_endpoint.rs                 # GetEndpointUseCase (Input/Output)
      update_endpoint.rs              # UpdateEndpointUseCase (Input/Output)
      delete_endpoint.rs              # DeleteEndpointUseCase (Input)
      restore_endpoint.rs             # RestoreEndpointUseCase (Input)
    destinations/
      mod.rs                          # Re-exports destination usecases
      list_destinations.rs            # ListDestinationsUseCase (Input/Output)
      create_destination.rs           # CreateDestinationUseCase (Input/Output)
      update_destination.rs           # UpdateDestinationUseCase (Input/Output)
      delete_destination.rs           # DeleteDestinationUseCase (Input)
    events/
      mod.rs                          # Re-exports event usecases
      list_events.rs                  # ListEventsUseCase (Input/Output)
      get_event.rs                    # GetEventUseCase (Input/Output)
      replay_event.rs                 # ReplayEventUseCase (Input/Output)
    ingestion/
      mod.rs                          # Re-exports ingestion usecases
      receive_webhook.rs              # ReceiveWebhookUseCase (Input/Output/RateLimitInfo)
    playground/
      mod.rs                          # Re-exports playground usecases
      create_playground.rs            # CreatePlaygroundUseCase (Input/Output)
      resume_playground.rs            # ResumePlaygroundUseCase (Input/Output)
      migrate_playground.rs           # MigratePlaygroundUseCase (Input/Output)
    subscription/
      mod.rs                          # Re-exports subscription usecases
      get_subscription.rs             # GetSubscriptionUseCase (Input/Output)
    websocket/
      mod.rs                          # Re-exports websocket usecases
      handle_delivery_result.rs       # HandleDeliveryResultUseCase (Input/Output)
      dispatch_event.rs               # DispatchEventUseCase
    background/
      mod.rs                          # Re-exports background usecases
      delivery_timeout.rs             # DeliveryTimeoutUseCase - marks stuck in_progress deliveries as Timeout
      rate_limit_cleanup.rs           # RateLimitCleanupUseCase - deletes old rate limit records
      playground_cleanup.rs           # PlaygroundCleanupUseCase - cleans up expired playground sessions
      event_expiry.rs                 # EventExpiryUseCase - deletes expired events
  handlers/
    mod.rs                            # Declares all handler modules
    app.rs                            # AppState, start(), spawn_background_tasks(), middleware, router assembly
    extractors/
      mod.rs                          # Re-exports AuthenticatedUser, PlaygroundUser
      auth.rs                         # AuthenticatedUser: Supabase JWT extractor (Authorization: Bearer <token>)
      playground.rs                   # PlaygroundUser: anonymous token extractor (X-Playground-Token header)
    routers/
      mod.rs                          # Re-exports ApiError + all router modules
      error_response.rs               # ApiError(UsecaseError) implementing IntoResponse
      default_routers.rs              # server_health_check, db_health_check, not_found
      endpoints/
        mod.rs                        # pub fn router() -> Router<AppState> with CRUD + restore routes
        create.rs                     # create_endpoint handler
        list.rs                       # list_endpoints handler
        get.rs                        # get_endpoint handler
        update.rs                     # update_endpoint handler
        delete.rs                     # delete_endpoint handler
        restore.rs                    # restore_endpoint handler
      destinations/
        mod.rs                        # pub fn router() -> Router<AppState> with CRUD routes
        list.rs                       # list_destinations handler
        create.rs                     # create_destination handler
        update.rs                     # update_destination handler
        delete.rs                     # delete_destination handler
      events/
        mod.rs                        # pub fn router() -> Router<AppState> with list/get/replay routes
        list.rs                       # list_events handler
        get.rs                        # get_event handler
        replay.rs                     # replay_event handler
      ingestion/
        mod.rs                        # pub fn router() -> Router<AppState> with catch-all webhook receive
        receive.rs                    # receive_webhook handler (any HTTP method)
      playground/
        mod.rs                        # pub fn router() -> Router<AppState> with create/resume/migrate
        create.rs                     # create_playground handler
        resume.rs                     # resume_playground handler
        migrate.rs                    # migrate_playground handler
      subscription/
        mod.rs                        # pub fn router() -> Router<AppState> with get
        get.rs                        # get_subscription handler
      websocket/
        mod.rs                        # pub fn router() -> Router<AppState> with WS connect
        handler.rs                    # ws_connect handler (WebSocket upgrade)
        messages.rs                   # WebSocket message types (client <-> server)
    delivery_timeout/
      mod.rs                          # spawn() for delivery timeout background task loop
    rate_limit/
      mod.rs                          # spawn() for rate limit cleanup background task loop
    playground_cleanup/
      mod.rs                          # spawn() for playground cleanup background task loop
    event_expiry/
      mod.rs                          # spawn() for event expiry background task loop
  infra/
    mod.rs                            # Declares db, ws modules
    db/
      mod.rs                          # Re-exports postgres_connection, repositories, schema
      postgres_connection.rs          # PgPool type alias (deadpool), create_pool()
      schema.rs                       # Diesel auto-generated (do not edit)
      repositories/
        mod.rs                        # Re-exports all Postgres impls
        error_mapping.rs              # map_diesel_error(), map_pool_error() centralized helpers
        endpoint_postgres.rs          # EndpointPostgres: EndpointRow, NewEndpointRow, into_entity()
        destination_postgres.rs       # DestinationPostgres
        event_postgres.rs             # EventPostgres
        delivery_postgres.rs          # DeliveryPostgres
        subscription_postgres.rs      # SubscriptionPostgres
        playground_session_postgres.rs # PlaygroundSessionPostgres
        rate_limit_postgres.rs        # RateLimitPostgres
        replay_quota_postgres.rs      # ReplayQuotaPostgres
        provider_preset_postgres.rs   # ProviderPresetPostgres
    ws/
      mod.rs                          # Re-exports WsRegistry, WsConnection, WsMessage
      registry.rs                     # WsRegistry (DashMap<EndpointId, Vec<Arc<WsConnection>>>)
```

## Decision defaults
- Usecases use `Arc<dyn Repo>` trait objects (not generics).
- Repository implementations hold `Arc<PgPool>` (deadpool).
- Error mapping uses centralized `map_diesel_error()` and `map_pool_error()` helpers.
- Repos created in handlers: `Arc::new(EntityPostgres::new(Arc::clone(&state.db_pool)))`.
- Background tasks: handler spawns loop (`spawn()` -> `JoinHandle<()>`), usecase has logic, repo does DB query.
- Router organization: each domain gets a `routers/{domain}/mod.rs` with `pub fn router() -> Router<AppState>`.
- Two auth extractors: `AuthenticatedUser` (Supabase JWT) for API routes, `PlaygroundUser` (anonymous token header) for playground routes.
- WebSocket connections tracked in `WsRegistry` (in-memory DashMap), keyed by `EndpointId`.
- Config uses `BackgroundTasks::default()` with production-ready values; `env_or()` for optional overrides.

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
