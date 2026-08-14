# Optimize Repository / Database Level

## 1. Get query evidence

- [ ] Identify the exact query (sqlc query name → generated SQL)
- [ ] `EXPLAIN (ANALYZE, BUFFERS)` with real parameters and production-like data
- [ ] Check `pool.Stat()` — acquisition wait vs execution time (exhaustion masquerades as slow
      queries)

## 2. Candidate optimizations (smallest first)

- [ ] Add the missing index (goose migration); re-run EXPLAIN to confirm it is used
- [ ] Fix query shape: sargable predicates, needed columns only, pagination
- [ ] Fix N+1: add an intent-named batch method to the repository interface
      (`FindByIDs` with `= ANY($1)`, or a join query) — propose the interface change first
- [ ] Batch multi-statement writes with `pgx.Batch` inside the repository
- [ ] Bulk inserts with `CopyFrom` for thousands of rows
- [ ] Narrow transaction scope; move IO and business logic outside `Begin`/`Commit`
- [ ] Tune pool sizing by DB capacity (`MaxConns` ≈ 2–4× DB cores), lifetimes, and idle settings
- [ ] Reduce row-mapping allocations (preallocate result slices in `:many` mapping)

## 3. Keep the architecture

- [ ] SQL and pgx stay in `internal/infra/postgres`; handlers/usecases never touch queries
- [ ] New batch methods are domain interface methods; sqlc regenerated (`sqlc generate`)
- [ ] Migrations backward-compatible when practical

## 4. Verify

- [ ] Repository integration tests pass (`go test -race ./...`)
- [ ] Before/after: repository benchmark and/or EXPLAIN timings
      (`templates/repository-query-benchmark.md`)
- [ ] Endpoint-level improvement confirmed if the goal was API latency
