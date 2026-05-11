# Performance and Concurrency Review

## Scope

This reference is for code review level checks. It is not a benchmarking or profiling guide.

The goal is to catch:

- deadlock risks
- runtime starvation
- lock misuse
- unbounded concurrency
- unbounded memory growth
- task lifecycle leaks
- resource exhaustion
- avoidable hot-path allocations
- obvious Rust async performance footguns

## Async Runtime Safety

Check:

- no `std::thread::sleep` in async code
- use `tokio::time::sleep` instead of `std::thread::sleep` in async code
- no blocking recv/send from sync channels in async tasks
- no blocking filesystem/network/database operations on async runtime threads unless
  project-approved
- use `spawn_blocking` for short-lived blocking work
- use a dedicated thread/worker for long-lived blocking loops
- bound CPU-heavy `spawn_blocking` work with semaphore or dedicated executor
- avoid CPU-heavy loops in request handlers

## Locking

Check:

- no `MutexGuard`/`RwLockGuard` held across `.await`
- no `RefCell` borrow held across `.await`
- lock scope is minimal
- no nested lock deadlock risk
- lock order is consistent
- `std::sync::Mutex` is acceptable only for short, low-contention critical sections with no await
- `tokio::sync::Mutex` is used only when a lock must be held across await or async resource access
  requires it
- consider task ownership/message passing for async IO resources

## Channels and Backpressure

Check:

- avoid unbounded channels unless memory growth is bounded elsewhere
- bounded channels should define behavior when full
- producers cannot overwhelm consumers indefinitely
- retry loops have cap/backoff/timeout
- background queues expose failure/overflow behavior
- request path cannot enqueue unlimited work

## Task Lifecycle

Check:

- `JoinHandle` is awaited, tracked, or intentionally detached
- detached tasks are documented and safe
- long-running tasks have cancellation/shutdown path
- task errors are logged or propagated
- background workers do not silently die
- `select!` loops do not starve lower-priority branches
- shutdown does not drop important work silently

## Shared State

Check:

- avoid `Arc<Mutex<T>>` by default
- prefer immutable `Arc<T>` where mutation is not needed
- prefer ownership transfer/message passing for state with async IO
- prefer short critical sections for simple in-memory state
- watch for DB pool or API client contention under concurrency
- tenant/user boundary is preserved under concurrent access

## Allocation and Clone Review

Check:

- unnecessary `clone()` on request hot path
- unnecessary `to_string()`/`format!` in loops
- avoid `collect()` if streaming/iterator is enough
- pre-allocate `Vec`/`String` capacity when size is known and path is hot
- avoid cloning large domain entities just to satisfy ownership when borrowing or `Arc` is clearer
- avoid repeated serialization/parsing inside loops

## Reporting

Only flag performance issues when there is a plausible cost:

- per request
- inside a loop
- under a lock
- in a background worker
- in high concurrency code
- in DB/API hot path
- in a queue/worker path

Do not block on micro-optimizations without evidence. Recommend benchmarking/profiling when the
performance impact is uncertain or workload-dependent.
