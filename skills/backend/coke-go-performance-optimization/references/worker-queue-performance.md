# Worker and Queue Performance

## Check for

- Unbounded queue (buffered channel sized "big enough", slices used as queues).
- Unbounded concurrency (goroutine per job with no limit).
- No backpressure — producers never block or shed when consumers fall behind.
- No retry cap/backoff/jitter.
- No per-job timeout; no cancellation path.
- No graceful shutdown (jobs killed mid-flight, or shutdown hangs forever).
- Missing idempotency for retried jobs.
- DB pool exhaustion from worker concurrency exceeding `MaxConns`.
- Worker goroutine silently dying (panic swallowed, error loop exit) — throughput drops to zero
  without a crash.
- Queue memory growth; per-job allocation hot path; too much work under a shared lock.

## Guidance

- Use bounded channels/queues; define queue-full behavior explicitly (block, shed, or spill).
- Bound concurrency with a worker count or semaphore; size by downstream capacity (DB pool, API
  rate limits), not by job count.
- Channel size is one or none unless the chosen size has a written justification.
- Workers follow the owned-goroutine pattern: `Start`/`Stop`, stop signal, exit wait
  (`coke-go-clean-architecture` `templates/background_job.go`).
- Use idempotency keys for retryable jobs when relevant; capped exponential backoff with jitter.
- Every job runs under `context.WithTimeout`; shutdown cancels and waits.
- Instrument queue lag, job duration, and success/failure counts before tuning — the bottleneck is
  usually the downstream dependency, not the loop.
- Verify no leak: goroutine profile flat after load; `goleak` in worker tests.
- Add a throughput benchmark or synthetic worker test only when useful.
- Keep business logic in usecase/domain, not the worker loop.
