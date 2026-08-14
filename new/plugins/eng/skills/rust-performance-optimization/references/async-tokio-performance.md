# Async and Tokio Performance

Check for:

- Blocking work inside async handlers/tasks.
- `std::thread::sleep` in async code.
- Blocking filesystem/network/database calls on runtime worker threads.
- CPU-heavy work directly in async tasks.
- `spawn_blocking` used for many CPU-bound computations without limiting parallelism.
- Long-lived blocking loops inside `spawn_blocking`.
- Ignored `JoinHandle`.
- Background tasks without cancellation.
- Retry loops without timeout/backoff/cap.
- External IO without timeout.
- `select!` branch starvation.
- Unbounded task spawning.

Guidance:

- Use `tokio::time::sleep` in async code.
- Use `spawn_blocking` for short-lived blocking operations that finish on their own.
- For many CPU-bound tasks, use a semaphore, bounded worker pool, Rayon, or a dedicated executor.
- For long-lived blocking loops, prefer a dedicated thread or worker architecture.
- Define cancellation/shutdown behavior for long-running tasks.
- Track or intentionally detach `JoinHandle`.
- Use timeouts around external IO when appropriate.
- Use tracing spans to understand async latency.
