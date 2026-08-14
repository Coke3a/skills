# Go Profiling Tools

Choose the smallest useful tool for the observed issue:

- **CPU profile**: `go tool pprof 'http://localhost:6060/debug/pprof/profile?seconds=30'` — where
  CPU time goes.
- **Heap profile**: `inuse_space` for "why is RSS high"; `alloc_space` for "what churns the GC".
- **Goroutine profile**: leak hunting — take two snapshots minutes apart; a monotonically growing
  stack signature is the leak.
- **Block profile** (`runtime.SetBlockProfileRate`) and **mutex profile**
  (`runtime.SetMutexProfileFraction`): channel/lock contention; off by default, enable temporarily.
- **Execution trace**: `curl -o trace.out 'localhost:6060/debug/pprof/trace?seconds=5'` then
  `go tool trace trace.out` — scheduler latency, blocked goroutines, GC pause timing. Go 1.25's
  `runtime/trace.FlightRecorder` keeps an always-on ring buffer to dump on rare events (SLO
  breach) in production.
- **Load generators**: k6 (scenarios, CI thresholds), vegeta (constant-rate, avoids coordinated
  omission), hey (quick smoke), wrk (max raw throughput).
- **DB evidence**: `EXPLAIN (ANALYZE, BUFFERS)` on the actual query with production-like data.
- **Metrics/spans**: request, worker, and IO timing when latency is distributed across awaits/IO.

## Serving pprof

- Import `net/http/pprof` on a **separate internal port** (plain `http.Server` on `:6060`), never
  the public listener. Fiber projects can use the pprof middleware on an internal-only app.
- View with `go tool pprof -http=:8080 profile.pb.gz` (flamegraph + source view); CLI: `top`,
  `list Func`, `peek`. Compare runs with `-diff_base`.

## Capture Notes

- Profile under a realistic workload; capture during the load run, not after.
- Record tool, command, workload, build flags, Go version, and environment.
- Use allocation profiles for allocation rate/peak memory questions, not CPU-only profiles.
- `GODEBUG=gctrace=1` prints per-GC lines when GC share is the question.
- Escape analysis evidence: `go build -gcflags='-m'` (see `references/allocation-and-escape.md`).
