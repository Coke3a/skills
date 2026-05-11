# Locking and Shared State

Check for:

- `MutexGuard` / `RwLockGuard` held across `.await`.
- `RefCell` borrow held across `.await`.
- Broad lock scope.
- Nested lock deadlock risk.
- Lock contention on request hot path.
- `Arc<Mutex<T>>` used by default.
- `tokio::sync::Mutex` used unnecessarily.
- Shared mutable state around async IO.

Guidance:

- Use `std::sync::Mutex` only for short, low-contention critical sections with no await.
- Do not hold `std::sync::MutexGuard` across `.await`.
- Use `tokio::sync::Mutex` only when a lock must be held across await or async resource access requires it.
- Prefer task ownership and message passing for shared resources that do async IO.
- Consider sharding locks only when contention is real and measured.
- Keep critical sections short.
- Do not optimize locks by weakening correctness.
