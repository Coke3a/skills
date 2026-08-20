# Database Performance (pgx / sqlc)

A missing index beats any Go-side tuning by orders of magnitude — check the query plan first.

## Check for

- N+1 query pattern (usecase looping over repository calls).
- Query shape that prevents index use; missing indexes for frequent filter/order/join.
- Missing pagination; loading unused columns or too many rows.
- Chatty loops of single-row INSERT/UPDATE where batching would help.
- Transaction scope that is too broad (network calls or business logic inside `Begin`/`Commit`).
- Slow connection acquisition or pool exhaustion (`pool.Stat()`: `AcquireDuration`,
  `EmptyAcquireCount`).
- Missing context deadline on the request path.
- Expensive row → entity mapping in `:many` loops.

## Guidance

- Keep DB-specific work in infra/repository; do not let handlers or usecases touch SQL.
- `EXPLAIN (ANALYZE, BUFFERS)` the real query with production-like data when query time is the
  suspected bottleneck; add the index or fix the query shape in a golang-migrate migration.
- Fix N+1 by adding an intent-named batch method to the repository interface
  (`FindByIDs`, `ListWithChildren`) when the N+1 behavior is measured — propose the interface
  change, implement it with one sqlc query (join or `= ANY($1)`).
- Batch round trips with `pgx.Batch`/`SendBatch` inside the repository for multi-statement writes.
- Bulk-insert thousands of rows with `CopyFrom` (COPY protocol) — 10–100× looped INSERTs.
- pgx caches prepared statements automatically — do not hand-prepare; behind a transaction pooler
  (PgBouncer) switch `QueryExecMode` accordingly.
- Size `MaxConns` by database capacity (~2–4× DB cores), not app concurrency; pool exhaustion
  masquerades as app latency.
- Keep migrations for index/schema changes backward-compatible when practical.
- Add repository integration tests when query behavior changes.
- sqlc regeneration (`sqlc generate`) after query changes; generated types still must not leak
  past infra.
