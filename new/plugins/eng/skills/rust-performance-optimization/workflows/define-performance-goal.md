# Define Performance Goal

## 1. Clarify target

- [ ] Is the issue latency, throughput, CPU, memory, DB, worker queue, or async runtime?
- [ ] What operation or endpoint is affected?
- [ ] What workload is realistic?
- [ ] What is the current baseline?
- [ ] What is the target?
- [ ] What is the acceptable tradeoff?

## 2. Identify hot path candidate

- [ ] Handler/request path.
- [ ] Usecase orchestration.
- [ ] Domain calculation/validation.
- [ ] Repository/DB query.
- [ ] Worker/queue loop.
- [ ] Serialization/deserialization.
- [ ] External IO.

## 3. Decide measurement method

- [ ] Criterion benchmark.
- [ ] Integration benchmark.
- [ ] Profile.
- [ ] DB EXPLAIN/EXPLAIN ANALYZE.
- [ ] Tracing/metrics.
- [ ] Existing production metrics.

## 4. Architecture guardrail

- [ ] State which layer owns the optimization.
- [ ] Confirm no layer boundary will be bypassed.
- [ ] If architecture change is needed, propose it explicitly before implementation.
