# GC and Memory

Tune the GC only **after** reducing allocations — profile first.

## When GC tuning matters

- CPU profile shows meaningful `runtime.gcBgMarkWorker` / GC share (>5–10%).
- `GODEBUG=gctrace=1` shows frequent GC cycles under load.
- RSS approaches the container limit and OOM kills occur.

## Knobs

- **GOGC** (default 100): heap may grow 100% over the live set before the next GC. Raising it
  (200–400) trades memory for less GC CPU — appropriate for high-allocation-rate services with
  memory headroom.
- **GOMEMLIMIT** (soft limit): set to ~85–90% of the container memory limit
  (`GOMEMLIMIT=900MiB` for a 1GiB pod). Prevents OOM kills from GOGC letting the heap balloon and
  makes raising GOGC safe. `GOGC=off` + `GOMEMLIMIT` maximizes throughput but risks a GC death
  spiral near the limit — only with generous headroom.
- **GOMAXPROCS**: Go 1.25+ respects cgroup CPU limits automatically; on older Go in Kubernetes use
  `go.uber.org/automaxprocs` to avoid CFS throttling.

## Memory growth vs leak

- A heap that plateaus after GOGC-driven growth is not a leak.
- Real leak suspects: goroutine leaks pinning memory (see `references/goroutine-concurrency.md`),
  global caches without eviction, slices re-sliced from huge backing arrays, forgotten
  `time.Ticker`s, pooled objects retaining large buffers.
- Evidence: heap profile `inuse_space` diff over time; goroutine profile growth; `pool.Stat()` for
  connection-shaped "leaks".

## Rules

- Change one knob at a time and re-measure under the same load.
- Record GOGC/GOMEMLIMIT values in the before/after report; they are deploy-time configuration,
  not code, and must be captured or the result is unreproducible.
- Do not set GOMEMLIMIT below realistic peak live-set — that trades OOM for constant GC thrash.
