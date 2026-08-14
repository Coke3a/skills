# Database Tool Choice: sqlc + pgx

This architecture mandates **sqlc (with `sql_package: "pgx/v5"`) + pgx/pgxpool**, with **goose**
for migrations. This file records why, so the decision can be re-evaluated deliberately instead of
re-litigated per feature.

Selection priorities, in order: safety > no leaks > readability > performance.

## Why sqlc + pgx wins on those priorities

1. **Safety** — sqlc compiles queries against the real schema at generate time: a wrong column
   name, type, or arity fails the build, not production. Every query is a constant with numbered
   placeholders, so SQL injection is structurally impossible in generated code. There are no
   silent-error semantics (unlike GORM's `Find` returning nil error on zero rows).
2. **No leaks** — the generator emits `rows.Close()`/`rows.Err()` handling correctly every time;
   everything is context-first, and `pgxpool` manages connection lifecycle with health checks,
   lifetime jitter, and cancellation-aware cleanup.
3. **Readability** — the SQL is explicit and reviewable in `db/queries/*.sql`; repository code is
   thin typed calls (`q.GetExampleEntityByID(ctx, id)`), which maps cleanly onto the
   clean-architecture repository interface.
4. **Performance** — effectively raw pgx, the fastest PostgreSQL path in Go, with free access to
   `CopyFrom` (COPY), `SendBatch`, and LISTEN/NOTIFY when needed.

Known tradeoffs: a codegen step in the workflow (`sqlc generate`), and friction for truly dynamic
queries — for a rare dynamic-filter endpoint, hand-build the query with a small builder inside
infra, executed on the same pool, and keep it behind the repository interface.

## The alternatives and when they would be acceptable

| Tool | Verdict for this architecture |
| ---- | ----------------------------- |
| GORM | Most popular, fastest to prototype — but runtime reflection, silent/inconsistent error semantics, implicit behavior (zero-value fields dropped from updates), and the slowest performance. Acceptable for throwaway prototypes and admin tools, not for this architecture's production services. |
| ent  | Excellent type safety via generated builders; best when the domain is genuinely graph-shaped (heavy many-to-many traversal, GraphQL). The generated DSL hides the executed SQL — a reviewability cost this architecture avoids. |
| Bun  | Solid SQL-first builder, near-pgx performance; correctness still runtime-checked. Choose only when multi-database support is a hard requirement. |
| sqlx | Effectively dormant (last release 2024); runtime tag matching; manual rows handling. Do not use for new services. |
| raw pgx only | Fine for infra-internal plumbing (LISTEN/NOTIFY, COPY); as the main data path it gives up sqlc's compile-time checking for no gain. |

## Standing rules

- Repository implementations use sqlc-generated queries; hand-written SQL strings appear only for
  features sqlc cannot express, live only inside `internal/infra/postgres`, and use parameterized
  arguments — never string concatenation.
- Migrations are goose files in `db/migrations/`; sqlc reads them as its schema, which keeps
  schema and queries verifiably in sync.
- Re-evaluate this choice only at project level, with this file updated — not ad hoc inside a
  feature.
