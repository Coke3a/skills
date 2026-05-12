# Worker and Queue Performance

Check for:

- Unbounded queue.
- Unbounded concurrency.
- No backpressure.
- No retry cap/backoff/jitter.
- No timeout.
- No cancellation.
- No graceful shutdown.
- Missing idempotency for retry.
- DB pool exhaustion.
- Task leak or worker silently dying.
- Queue memory growth.
- Per-job allocation/clone hot path.
- Too much work under lock.

Guidance:

- Use bounded channels/queues where practical.
- Define queue-full behavior.
- Bound concurrency with semaphore or worker count.
- Use idempotency keys for retryable jobs when relevant.
- Use capped exponential backoff with jitter when appropriate.
- Use tracing/metrics for queue lag, job duration, and success/failure counts.
- Add throughput benchmark or synthetic worker test only when useful.
- Keep business logic in usecase/domain, not the worker loop.
