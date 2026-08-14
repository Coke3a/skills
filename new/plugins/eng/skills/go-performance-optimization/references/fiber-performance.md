# Fiber / fasthttp Performance

Fiber is fast by default because fasthttp reuses buffers — which is also its main correctness
trap. Never trade the safety rules below for speed.

## The buffer-reuse rule (correctness before speed)

Everything from `c.Params()`, `c.Query()`, `c.Body()`, `c.Get()`, `c.FormValue()` is only valid
until the handler returns. Storing it, sending it to a channel, or passing it to a goroutine
without `utils.CopyString`/`utils.CopyBytes`/`strings.Clone` causes silent data corruption under
load — a bug that looks exactly like a "performance mystery" (wrong cache keys, cross-request data
bleed). Audit for this **before** optimizing anything.

- `fiber.Config{Immutable: true}` makes all context values safe at the cost of an allocation per
  access — reasonable for CRUD services; avoid on measured hot paths, where targeted copies win.
- Binding into DTOs (`c.Bind().Body(&req)`) copies naturally and is the preferred pattern.

## Check for

- JSON encode/decode dominating CPU profiles (consider a faster `JSONEncoder`/`JSONDecoder` in
  `fiber.Config` — e.g. goccy/go-json or sonic — only with benchmarks and only if correctness
  constraints allow).
- Middleware doing per-request allocation/formatting that could be conditional or cheaper
  (logger templates, verbose request dumps left on in production).
- Missing `compress` middleware for large text responses (or compressing tiny/binary responses —
  also waste).
- Rate limiter / session middleware using default in-memory storage across replicas — correctness
  issue that also burns memory.
- `BodyLimit` far above real payload sizes (memory amplification under attack/load).
- Handlers doing work that belongs in usecases — layer violations disguised as "inlining for
  speed".

## Guidance

- Timeouts are throughput protection: `ReadTimeout`/`WriteTimeout`/`IdleTimeout` always set —
  slowloris connections otherwise pin memory and file descriptors.
- **Prefork**: off by default. It helps only for extreme connection counts, breaks shared
  in-process state (caches, limiters), and complicates containers. Scale with replicas; enable
  prefork only with a load-test proving the win.
- Keep handlers thin; serialization cost belongs in the handler layer and is optimized there
  (response shape, field count), not by moving business logic in.
- Measure endpoint latency with a constant-rate load tool (vegeta/k6) and capture pprof during the
  run; Fiber-internal tweaks rarely dominate — DB and allocation usually do.
