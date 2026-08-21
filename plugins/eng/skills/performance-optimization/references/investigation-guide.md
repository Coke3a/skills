# Performance investigation guide

Choose the measurement that can falsify the suspected cause.

| Symptom | Start with | Deepen with |
| --- | --- | --- |
| Slow request or operation | End-to-end latency distribution | Trace, CPU profile, dependency timing |
| Low throughput | Load test at representative concurrency | Saturation, queue, lock, and scheduler evidence |
| High CPU | CPU sampling profile | Hot-loop benchmark and generated-code inspection |
| Memory growth | Time-series memory and object counts | Heap profile, retention paths, leak checks |
| Allocation or GC pressure | Allocation metrics under the real workload | Escape/allocation profile and focused benchmark |
| Slow database access | Query timing with representative parameters | Query plan, row estimates, I/O, locks, pool waits |
| Slow startup/build or large bundle | Phase timing or artifact composition | Dependency/module graph and per-stage comparison |

Prefer production-like evidence when safe, then reduce the problem to a repeatable local experiment. A microbenchmark is useful for testing an isolated hypothesis, but it does not replace an end-to-end check.

When comparing results:

- use multiple samples and a statistical comparison when the ecosystem supports it;
- keep inputs, build mode, hardware, concurrency, and cache state controlled;
- record absolute values as well as percentages;
- inspect tail latency and resource tradeoffs, not only averages;
- call the result inconclusive when noise overlaps the observed difference.
