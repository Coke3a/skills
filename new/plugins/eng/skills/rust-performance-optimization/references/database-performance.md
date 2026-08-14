# Database Performance

Check for:

- N+1 query pattern.
- Query shape that prevents index use.
- Missing indexes for frequent filter/order/join.
- Missing pagination.
- Loading unused columns or too many rows.
- Transaction scope that is too broad.
- Slow connection acquisition or DB pool exhaustion.
- Repeated insert/update where batching would help.
- Expensive row -> domain mapping or large clone cost.
- Missing DB timeout on request path.

Guidance:

- Keep DB-specific work in infra/repository.
- Use `EXPLAIN` / `EXPLAIN ANALYZE` when query time is the suspected bottleneck.
- Optimize query shape in infra.
- Add repository trait batch methods when usecase N+1 behavior is measured.
- Add migrations for index/schema changes.
- Keep migrations backward-compatible when practical.
- Add repository integration tests when query behavior changes.
- Do not let handlers call optimized SQL directly.
