# Optimize Goroutines / Workers

Use for goroutine leaks, unbounded concurrency, channel contention, and worker throughput.

## 1. Get evidence

- [ ] Goroutine profile: two snapshots minutes apart under load — growing stack signature = leak
- [ ] Block/mutex profiles for channel/lock contention (enable temporarily)
- [ ] Execution trace when goroutines look idle but latency is bad (scheduler view)
- [ ] Worker metrics: queue lag, jobs/sec, per-job duration, downstream saturation (DB pool)

## 2. Fix leaks first (they masquerade as performance problems)

- [ ] Every goroutine gets an owner: stop signal + exit wait
- [ ] Channel sends get a `ctx.Done()` escape
- [ ] `defer cancel()` after every `WithTimeout`/`WithCancel`; `Stop()` tickers
- [ ] Add `goleak.VerifyNone` to the relevant tests

## 3. Candidate optimizations (smallest first)

- [ ] Bound concurrency: worker pool / semaphore / `errgroup.SetLimit(n)` — size by
      `GOMAXPROCS` (CPU-bound) or downstream capacity (IO-bound), not by job count
- [ ] Bounded channels with explicit queue-full behavior; justify any buffer size > 1
- [ ] Reduce per-job allocation (see `workflows/optimize-allocation.md`)
- [ ] Move blocking work off shared paths; batch downstream calls per worker tick
- [ ] Backoff with cap and jitter on retries so retries stop amplifying load

## 4. Keep the architecture

- [ ] Business logic stays in usecases; the worker loop owns only scheduling/lifecycle
- [ ] Worker concurrency respects DB `MaxConns` — raising one without the other just moves the
      queue

## 5. Verify

- [ ] `go test -race ./...`; goroutine count flat after load
- [ ] Throughput before/after per `templates/worker-throughput-benchmark.md`
- [ ] Graceful shutdown still stops and waits
