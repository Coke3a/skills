# Optimize Async Worker or Queue

## 1. Identify issue

- [ ] Runtime starvation.
- [ ] Worker throughput.
- [ ] Queue lag.
- [ ] Lock contention.
- [ ] DB pool exhaustion.
- [ ] Unbounded task/channel growth.
- [ ] Missing cancellation.

## 2. Inspect

- [ ] Task spawning.
- [ ] JoinHandle handling.
- [ ] Channel bounds.
- [ ] Semaphore/worker count.
- [ ] Retry/backoff/timeout.
- [ ] Lock scope.
- [ ] DB/API concurrency.
- [ ] Shutdown behavior.

## 3. Optimize

- [ ] Bound concurrency.
- [ ] Add backpressure.
- [ ] Avoid blocking async runtime.
- [ ] Move CPU-heavy work to bounded blocking/Rayon/dedicated worker.
- [ ] Add cancellation/shutdown path.
- [ ] Shorten lock scope.
- [ ] Add tracing/metrics where useful.

## 4. Verify

- [ ] Run worker benchmark or synthetic workload if available.
- [ ] Run correctness tests.
- [ ] Summarize throughput/latency before/after.
