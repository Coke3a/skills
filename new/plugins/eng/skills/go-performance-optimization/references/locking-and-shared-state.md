# Locking and Shared State

## Check for

- Lock contention on the request hot path (mutex profile evidence, not intuition).
- Broad lock scope — IO, DB calls, or serialization performed while holding a lock.
- Nested lock acquisition and deadlock risk.
- `sync.RWMutex` write-lock held where a read-lock suffices (or vice-versa assumptions unproven).
- Maps/slices returned from behind a mutex without copying — callers race on the internals.
- Raw `sync/atomic` on plain ints where a non-atomic read can slip in — prefer typed
  `atomic.Bool`/`atomic.Int64` or `go.uber.org/atomic`.
- Embedded mutexes (exposes `Lock`/`Unlock` as API) — keep the mutex an unexported field.
- Shared mutable state where task ownership or message passing would remove the lock entirely.

## Guidance

- Keep critical sections short: compute outside, lock only around the state mutation.
- Zero-value `sync.Mutex` is valid; never `new(sync.Mutex)`; never copy a struct containing one.
- Copy data in and out at the boundary of the locked region (uber-go copy-at-boundaries rule).
- Enable the mutex/block profilers temporarily to *prove* contention before restructuring.
- Options in escalating order: shrink the critical section → RWMutex for read-heavy state →
  per-key/sharded locks → `sync.Map` (only for append-mostly, disjoint-key workloads) → redesign
  to task ownership + channels.
- Consider sharding only when contention is real and measured.
- `-race` stays on in tests throughout every locking change.
- Do not optimize locks by weakening correctness.

## After removing contention, re-check tail latency

Throughput and p99 can move in opposite directions. A contended mutex also acts as an accidental
FIFO admission queue: it serializes work, but it serializes it *fairly*. Remove it and every
in-flight goroutine competes under Go's deliberately unfair scheduler, so throughput can jump while
p99 barely improves or gets worse — a starved tail replaces a uniform wait.

When that happens the fix is explicit admission control, not putting the lock back: a
`chan struct{}` semaphore (or `errgroup.SetLimit`) sized to `GOMAXPROCS` for CPU-bound work, with a
`ctx.Done()` escape on acquire. Measure across several limit values; the right size is usually the
smallest one that keeps throughput flat.

Report the p50 cost honestly — admission control usually raises p50 under saturation while cutting
p99, and that trade is the user's call, not yours to hide.

## Caches are a measurement target, not a given

Before optimizing the lock around a cache, measure the cache itself: instrument hits and misses.
A cache whose key includes a per-request or per-entity unique value has a hit rate of zero, so it
is pure cost — lock contention, allocation, and unbounded memory growth in exchange for nothing.
Deleting it beats sharding it, and the deletion is justified by the hit-rate number rather than by
taste. An unbounded cache with no eviction is a memory leak wearing a useful-sounding name.
