# Verify Performance Change

## 1. Correctness

- [ ] Run relevant tests.
- [ ] Run `go test -race ./...`.
- [ ] Confirm behavior unchanged unless intentionally changed.

## 2. Architecture

- [ ] Handler remains thin.
- [ ] Usecase does not depend on infra/HTTP.
- [ ] Domain remains pure.
- [ ] Infra owns DB-specific optimization.
- [ ] No DTO / sqlc-generated type leakage.
- [ ] No bypassed repository interface.
- [ ] No fasthttp buffer aliasing introduced.
- [ ] Every new goroutine has stop signal + exit wait.

## 3. Performance

- [ ] **Re-measure the metric the user actually complained about**, on the same workload shape they
      described. Improving a proxy is not the same as fixing the symptom: a change can multiply
      throughput while leaving p99 — the original complaint — unchanged or worse. If the report
      says "p99", the after-numbers must include p99.
- [ ] Re-run the benchmark/profile with the same workload.
- [ ] Compare with benchstat (≥10 runs) or profile diff.
- [ ] Identify variance/uncertainty; benchstat `~` = inconclusive — report that honestly.
- [ ] Check the metrics you did not target for regressions (p50 when tuning p99, memory when
      tuning CPU, throughput when tuning latency) and report any trade you made.
- [ ] Record runtime config (GOGC, GOMEMLIMIT, pool sizes) if it changed.

## 4. Quality

- [ ] `go build ./...` and `go vet ./...` pass; `golangci-lint run` when configured.
- [ ] Request code review for non-trivial optimization.
- [ ] Document tradeoffs per `templates/before-after-report.md`.
