# Repository And sqlc/pgx Reference

Use this reference for repository interfaces and their sqlc/pgx implementations. For why this
stack was chosen, see `references/db-tool-choice.md`.

## Repository port pattern

- Define interfaces in `internal/domain/repository/{entity}_repository.go`.
- Keep repository interfaces as persistence ports only; do not put pgx, sqlc-generated, pool, or
  transaction types in domain.
- Implement interfaces in `internal/infra/postgres/{entity}_repository.go`.
- Use `internal/domain/service/` for external-service ports that are not persistence concerns.
- Every method takes `context.Context` as its first parameter.
- Methods return `error`; the sentinels in `internal/domain/repository/errors.go` are the contract.
- `FindBy...` methods return `(*entity.T, error)` and a wrapped `repository.ErrNotFound` when no
  row matches — never `(nil, nil)`.
- Updates/deletes that expect an existing row return `repository.ErrNotFound` when zero rows are
  affected.
- List/search methods return `([]*entity.T, error)` with an empty slice when nothing matches — an
  empty result is a valid result, not `ErrNotFound`.

```go
type ExampleRepository interface {
	Create(ctx context.Context, e *entity.ExampleEntity) error
	FindByID(ctx context.Context, id valueobject.ExampleEntityID) (*entity.ExampleEntity, error)
	Update(ctx context.Context, e *entity.ExampleEntity) error
	Delete(ctx context.Context, id valueobject.ExampleEntityID) error
}
```

## Repository responsibility boundary

A repository translates and returns. It does not decide what a failure *means* — that is the
usecase's job, and only the usecase has the context to do it.

A repository must not:

- **Log.** It cannot tell whether a failure is a problem: `ErrNotFound` from `FindByID` is a 404 in
  one usecase and the normal "then create it" path in another.
- **Retry, back off, or fall back** to another source.
- **Swallow an error** and return a zero value, `nil`, or an empty slice in its place.
- **Return `(nil, nil)`** — the sentinel contract above exists to make absence explicit.
- **Decide business semantics from a driver error**, such as treating a unique violation as
  "already exists, that's fine".
- **Call other repositories, services, or usecases** to complete an operation. Multi-write atomicity
  is expressed as one intent-named method on this repository (see Transactions below), not as
  orchestration inside infra.

What it does instead: run the query, map the driver error onto a `repository.Err*` sentinel with
`mapPgError`, convert rows to entities, return. Everything else travels up intact so the usecase can
decide. See the ownership table in `references/error-handling.md`.

## sqlc project setup

- `sqlc.yaml` at the project root; `db/migrations/` (goose) is the schema source of truth;
  `db/queries/*.sql` holds the queries.
- Generated code goes to `internal/infra/postgres/gen` (package `gen`) with
  `sql_package: "pgx/v5"`. Never edit generated files.
- Run `sqlc generate` after every migration or query change and commit the generated diff.
- sqlc verifies each query against the schema at generate time — a wrong column name or type
  fails the build instead of production. This is the core safety property of the stack; do not
  bypass it with hand-written SQL strings in repositories.

Query annotations:

| Annotation  | Use for                                       |
| ----------- | --------------------------------------------- |
| `:one`      | Single-row reads (`pgx.ErrNoRows` if missing) |
| `:many`     | Multi-row reads                               |
| `:exec`     | Writes where affected count is irrelevant     |
| `:execrows` | Writes that must detect zero rows affected    |

## Implementation pattern

- Name the implementation `{Entity}Repository` in package `postgres`.
- Add the compile-time interface check.
- Wrap `gen.Queries`; construct from `*pgxpool.Pool`.
- Use centralized `mapPgError(op, err)`.
- Keep generated types private to the `postgres` package.
- Return domain entities or value objects, never generated rows or DTOs.

```go
var _ repository.ExampleRepository = (*ExampleRepository)(nil)

type ExampleRepository struct {
	q *gen.Queries
}

func NewExampleRepository(pool *pgxpool.Pool) *ExampleRepository {
	return &ExampleRepository{q: gen.New(pool)}
}
```

## Row conversion pattern

One unexported helper converts a generated row into a domain entity through `...FromExisting` and
`...FromTrusted` constructors. Parsing failures (for example an unknown status value in the
database) are real errors — return them, do not panic.

```go
func rowToExampleEntity(row gen.ExampleEntity) (*entity.ExampleEntity, error) {
	status, err := valueobject.ParseExampleEntityStatus(row.Status)
	if err != nil {
		return nil, fmt.Errorf("example_entity.map_row: %w", err)
	}

	var deletedAt *time.Time
	if row.DeletedAt != nil {
		t := *row.DeletedAt
		deletedAt = &t
	}

	return entity.ExampleEntityFromExisting(
		valueobject.ExampleEntityIDFromTrusted(row.ID),
		row.OwnerID,
		valueobject.ExampleEntityNameFromTrusted(row.Name),
		row.Url,
		status,
		row.CreatedAt,
		row.UpdatedAt,
		deletedAt,
	), nil
}
```

Row field types assume the `templates/sqlc.yaml` overrides (`uuid` → `uuid.UUID`, `timestamptz` →
`time.Time`, nullable `timestamptz` → `*time.Time`). Without overrides sqlc emits `pgtype.*`
wrappers — adapt the conversion helper, not the domain.

## Error mapping

```go
// internal/infra/postgres/errors.go — shared by all repositories in the package.
func mapPgError(op string, err error) error {
	if errors.Is(err, pgx.ErrNoRows) {
		return fmt.Errorf("%s: %w", op, repository.ErrNotFound)
	}

	var pgErr *pgconn.PgError
	if errors.As(err, &pgErr) {
		switch pgErr.Code {
		case "23505":
			return fmt.Errorf("%s: %s: %w", op, pgErr.ConstraintName, repository.ErrUniqueViolation)
		case "23503":
			return fmt.Errorf("%s: %s: %w", op, pgErr.ConstraintName, repository.ErrForeignKeyViolation)
		}
	}

	return fmt.Errorf("%s: %w", op, err)
}
```

Operation names follow `entity.operation`:

- `example_entity.create`
- `example_entity.find_by_id`
- `example_entity.update`
- `example_entity.delete`

## Query patterns

### Create

```go
func (r *ExampleRepository) Create(ctx context.Context, e *entity.ExampleEntity) error {
	err := r.q.CreateExampleEntity(ctx, gen.CreateExampleEntityParams{ /* from entity getters */ })
	if err != nil {
		return mapPgError("example_entity.create", err)
	}
	return nil
}
```

### Find by ID

```go
func (r *ExampleRepository) FindByID(
	ctx context.Context,
	id valueobject.ExampleEntityID,
) (*entity.ExampleEntity, error) {
	row, err := r.q.GetExampleEntityByID(ctx, id.UUID())
	if err != nil {
		return nil, mapPgError("example_entity.find_by_id", err)
	}
	return rowToExampleEntity(row)
}
```

### Update / delete with existence check

Use `:execrows` so zero affected rows becomes `ErrNotFound`.

```go
affected, err := r.q.UpdateExampleEntity(ctx, params)
if err != nil {
	return mapPgError("example_entity.update", err)
}
if affected == 0 {
	return fmt.Errorf("example_entity.update: id %s: %w", e.ID(), repository.ErrNotFound)
}
```

## Transactions

Use a transaction only when one usecase requires multiple writes to commit atomically. Keep
transaction control inside infra — expose an intent-named repository method
(`CreateWithMemberships`), not a generic `WithTx` on the domain interface.

```go
func (r *ExampleRepository) CreateWithChildren(ctx context.Context, e *entity.ExampleEntity, children []*entity.Child) error {
	tx, err := r.pool.Begin(ctx)
	if err != nil {
		return mapPgError("example_entity.tx_begin", err)
	}
	defer tx.Rollback(ctx) // no-op after successful Commit

	q := r.q.WithTx(tx)
	// ... q.CreateExampleEntity, q.CreateChild ...

	if err := tx.Commit(ctx); err != nil {
		return mapPgError("example_entity.tx_commit", err)
	}
	return nil
}
```

Rules:

- `defer tx.Rollback(ctx)` immediately after `Begin` — every early return is then safe.
- No network calls, provider SDK calls, or business decisions inside an open transaction.
- Keep transaction scope minimal; long-idle transactions bloat PostgreSQL and pin pool connections.

## Connection pool

One `*pgxpool.Pool` is created at startup in `internal/infra/postgres/pool.go`, injected into
repositories, and closed on shutdown. Never connect per request.

```go
cfg, err := pgxpool.ParseConfig(databaseURL)
// starting points — tune with pool.Stat() metrics:
cfg.MaxConns = 25                              // respect PG max_connections across replicas
cfg.MinConns = 5
cfg.MaxConnLifetime = time.Hour
cfg.MaxConnLifetimeJitter = 5 * time.Minute    // avoid mass reconnect storms
cfg.MaxConnIdleTime = 15 * time.Minute
cfg.HealthCheckPeriod = 30 * time.Second
pool, err := pgxpool.NewWithConfig(ctx, cfg)
```

## Leak prevention rules

- Every query runs under a context with a deadline — the request context from Fiber plus a DB-level
  cap where the project defines one. pgx cancels the query server-side on context cancellation.
- sqlc-generated code closes rows and checks `rows.Err()` — one reason it is the mandated path.
  Hand-written `pool.Query` calls (rare, and only inside infra) must `defer rows.Close()` and check
  `rows.Err()` after the loop.
- `defer tx.Rollback(ctx)` after every `Begin`.
- The pool is a singleton owned by `main()`; `defer pool.Close()` in `run()` closes it after
  `Listen` returns. Do not close it from a Fiber `OnPostShutdown` hook — see the shutdown notes in
  `references/architecture.md`.

## Migrations

- Use goose; migrations live in `db/migrations/` and double as the sqlc schema source.
- One migration per schema change, with `-- +goose Up` / `-- +goose Down` sections.
- Keep migrations backward-compatible when practical (additive first, destructive later).
