# Goroutines, Channels, and Leaks

Goroutines are cheap, not free: each costs stack memory and scheduler CPU, and a leaked goroutine
pins everything it references. The uber-go guide's strongest rule applies: no fire-and-forget.

## Check for

- Bare `go func()` without stop signal or exit wait.
- Goroutines started per request without a bound.
- Channel sends with no `ctx.Done()` escape (`ch <- v` that can block forever).
- Receivers that exited while senders still send (permanent block = leak).
- `time.Ticker`/`time.Timer` without `Stop`.
- Goroutines in `init()`.
- Missing `defer cancel()` after `context.WithTimeout`/`WithCancel`.
- Ignored errgroup/WaitGroup waits.
- Unbounded fan-out on collections (`for _, x := range items { go process(x) }`).
- `select` starvation of a rarely-ready branch.
- Blocking calls (file IO, CGO, syscalls) saturating all Ps under load.

## Guidance

- Every goroutine has an owner with a stop signal and a way to wait for exit
  (`stop`/`done` channels or `sync.WaitGroup`); shutdown signals stop **and waits**.
- Wrap channel sends in goroutines: `select { case ch <- v: case <-ctx.Done(): return }`.
- Bound concurrency: fixed worker pool over a channel, `chan struct{}` semaphore, or
  `errgroup.Group` with `SetLimit(n)`. Size by `GOMAXPROCS` for CPU-bound work, by downstream
  capacity (DB pool size, API rate limits) for IO-bound work.
- Channel size is one or none unless a written justification explains the chosen size and the
  queue-full behavior.
- Pass `context.Context` down the whole call chain; request cancellation must reach pgx.
- Detect leaks: `go.uber.org/goleak` in tests (`defer goleak.VerifyNone(t)`); in production,
  goroutine-profile snapshots over time — steady growth of one stack signature is the leak.
- Prefer message passing / task ownership for shared resources that do IO; locks for short
  in-memory critical sections (see `references/locking-and-shared-state.md`).
