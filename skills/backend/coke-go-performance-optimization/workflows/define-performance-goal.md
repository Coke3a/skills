# Define Performance Goal

Turn a vague complaint ("it's slow", "memory keeps growing") into a measurable goal.

## 1. Clarify the symptom

- [ ] What is slow / big / stuck — endpoint, query, worker, whole process?
- [ ] Latency, throughput, memory, CPU, GC time, goroutine growth, or queue lag?
- [ ] Under what workload does it appear (traffic level, data size, concurrency)?
- [ ] When did it start; what changed?

## 2. Define the metric

- [ ] Choose the metric: p95/p99 latency, RPS, allocs/op, RSS, GC CPU share, jobs/sec, goroutines
- [ ] Choose the workload that will measure it (realistic input shape and size)
- [ ] Record the current value — this is the baseline
- [ ] Set the target value and why it matters (SLO, cost, user experience)

## 3. Scope the investigation

- [ ] Which layer likely owns it (handler / usecase / domain / repository / worker / runtime)?
- [ ] Decide the measurement tool: `testing.B` benchmark, load test (vegeta/k6), pprof, trace
- [ ] Fill in `templates/performance-investigation.md`

## 4. Exit criteria

- [ ] Goal is a number under a defined workload, not an adjective
- [ ] Baseline recorded with the exact command used
- [ ] If no realistic workload can be defined, stop and say so — do not optimize blind
