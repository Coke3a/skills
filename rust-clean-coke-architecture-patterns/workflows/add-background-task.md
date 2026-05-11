# Add a background task

1) Create the usecase
- [ ] Add `src/usecases/background/{task_name}.rs`.
- [ ] Struct holds `Arc<dyn Repo>` dependency.
- [ ] Implement sweep/cleanup method with `Duration` and limit params.
- [ ] Re-export from `src/usecases/background/mod.rs`.
- [ ] Re-export from `src/usecases/mod.rs`.

2) Create the handler spawner
- [ ] Add `src/handlers/{task_name}/mod.rs`.
- [ ] Single `spawn()` function returning `JoinHandle<()>`.
- [ ] Takes `Arc<UseCase>`, `CancellationToken`, and config params.
- [ ] Uses `tokio::time::interval` + `tokio::select!` with `cancel.cancelled()`.
- [ ] Logs on start and on shutdown.

3) Wire in app.rs
- [ ] Add config params to `BackgroundTasks` struct in `config_model.rs` with doc comments.
- [ ] Add default values in `BackgroundTasks::default()`.
- [ ] Add env var loading in `config_loader.rs` using `env_or()`.
- [ ] Add `pub mod {task_name};` to `src/handlers/mod.rs`.
- [ ] Add `use super::{task_name};` in `app.rs`.
- [ ] Create repo and usecase in `spawn_background_tasks()`.
- [ ] Add spawn call to the returned `Vec<JoinHandle<()>>`.

4) Current background tasks (4 total)
- `delivery_timeout` -- marks stuck deliveries as Timeout
- `rate_limit` -- cleans up old rate limit records
- `playground_cleanup` -- cleans up expired playground sessions
- `event_expiry` -- deletes events past retention period
