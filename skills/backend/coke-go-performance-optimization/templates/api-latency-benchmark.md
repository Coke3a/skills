# API Latency Benchmark

Endpoint / scenario:

- ...

Load tool and command (prefer constant-arrival-rate: vegeta or k6):

```sh
# example
echo "GET http://localhost:8080/api/v1/example-entities" | vegeta attack -rate=100 -duration=60s | vegeta report
```

Environment (build flags, Go version, GOMAXPROCS, DB state, replica count):

- ...

Warm-up performed:

- ...

Results (report p50 / p95 / p99 — never mean only):

| Metric | Baseline | After |
| ------ | -------- | ----- |
| p50    |          |       |
| p95    |          |       |
| p99    |          |       |
| RPS    |          |       |
| Errors |          |       |

Profile captured during the run (pprof/trace file + top findings):

- ...

Notes / variance:

- ...
