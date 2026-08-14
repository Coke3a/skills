# Uber Go Style Guide — Distilled

Source: <https://github.com/uber-go/guide/blob/master/style.md>. This file distills the rules this
architecture relies on. The guide's own meta-rule applies: **above all else, be consistent** —
apply style changes at package level or larger.

Performance-specific rules (strconv over fmt, cached `[]byte` conversions, container capacity)
live in `coke-eng:go-performance-optimization`; the guide scopes them to hot paths only.

## Safety and correctness

- **Pass interfaces as values** — almost never use a pointer to an interface; the underlying data
  can still be a pointer.
- **Verify interface compliance at compile time** for types whose API contract is the interface:
  `var _ repository.ExampleRepository = (*ExampleRepository)(nil)`.
- **Zero-value mutexes are valid** — `var mu sync.Mutex`, never `new(sync.Mutex)`; keep mutexes as
  unexported non-pointer struct fields and **never embed them** (embedding makes `Lock`/`Unlock`
  part of the type's API).
- **Copy slices and maps at boundaries** — when storing a received slice/map or returning an
  internal one, copy it; otherwise callers can mutate state behind your back. This is why entity
  getters that expose slices return copies.
- **Defer for cleanup** — locks, files, `tx.Rollback`; multiple return paths make manual cleanup
  unreliable, and defer's nanosecond overhead is irrelevant outside proven hot loops.
- **Channel size is one or none** — any other buffer size needs written justification for how the
  size was chosen and what happens when it fills.
- **Start iota enums at one** so the zero value is not silently a valid variant — unless
  zero-as-default is intentional.
- **Use `time.Time` and `time.Duration`** — never bare ints for instants or periods; name units in
  serialized fields (`IntervalMillis`) when the type cannot cross the boundary.
- **Handle type assertion failures** — always the comma-ok form; the single-return form panics.
- **Don't panic** in production code; return errors. Panic only for unrecoverable startup
  programmer errors. In tests use `t.Fatal`.
- **Prefer `go.uber.org/atomic`** (or Go's typed `atomic.Bool`/`atomic.Int64`) over raw
  `sync/atomic` on plain ints, which invites accidentally non-atomic reads.
- **Avoid mutable globals** — inject dependencies through constructors; a `now func() time.Time`
  field beats swapping a global in tests.
- **Avoid embedding types in public structs** — it leaks implementation details and freezes type
  evolution; delegate explicitly to an unexported field.
- **Avoid shadowing predeclared identifiers** (`error`, `len`, `string`, ...).
- **Avoid `init()`** — and never start goroutines or do IO in it. Configuration loading is an
  explicit `loadConfig()` called from `main()`.
- **Exit only in `main()`, preferably once** — `if err := run(); err != nil { log.Fatal(err) }`;
  exits elsewhere skip defers and kill testability.
- **Field tags on all marshaled structs** — the serialized form is a cross-system contract;
  explicit `json:"..."` tags protect it from renames.

## Goroutine lifecycle

The guide's strongest language ("do not leak goroutines in production code"):

- **No fire-and-forget** — every goroutine must have a predictable stop time or a stop signal,
  *and* a way to block until it has exited (`done` channel or `sync.WaitGroup`).
- **No goroutines in `init()`** — background work belongs to an object (`Worker`) with
  `Start`/`Stop`, where `Stop` signals *and waits*.
- Test for leaks with `go.uber.org/goleak`.

```go
stop := make(chan struct{})
done := make(chan struct{})
go func() {
	defer close(done)
	for {
		select {
		case <-ticker.C:
			flush()
		case <-stop:
			return
		}
	}
}()
// shutdown: close(stop); <-done
```

## Error handling

Covered in depth in `references/error-handling.md`; the guide's core rules:

- Choose the error form by whether callers must match it and whether the message is static:
  `errors.New` / `fmt.Errorf` when no matching is needed; exported `Err...` sentinel for static
  matching via `errors.Is`; a custom `...Error` type for dynamic info via `errors.As`.
- Wrap with `%w` when callers should reach the cause; `%v` obfuscates it. Wrapped sentinels are
  API contract — document them.
- Keep wrap context succinct; drop "failed to" prefixes.
- Naming: `Err` prefix for sentinel values, `Error` suffix for types.
- **Handle each error once** — wrap-and-return or log-and-degrade, never both.

## Style essentials

- Line length: soft limit 99 characters.
- Group related declarations (`const (...)`, `var (...)`); imports in two groups — standard
  library, then everything else.
- Package names: short, lowercase, singular, no underscores; never `util`, `common`, `shared`.
- Alias imports only on conflict or when the package name differs from the path element.
- Order within a file: types/consts/vars, constructors, exported methods, the rest; group by
  receiver.
- **Reduce nesting** — guard clauses and early returns; keep the happy path at minimal indent.
- Drop unnecessary `else` when both branches just assign.
- Prefix unexported package-level vars with `_` (`_defaultTimeout`) — except unexported errors,
  which use the `err` prefix.
- `:=` when setting a value; `var` when the zero value is the point (`var filtered []int`).
- `nil` is a valid slice — return `nil` rather than `[]T{}`; check emptiness with `len(s) == 0`.
- Reduce variable scope (`if err := f(); err != nil`) unless it fights nesting reduction.
- No naked bool/literal parameters — comment them (`true /* isLocal */`) or use a named type.
- Raw string literals over escaped quotes.
- Struct init: always field names (except tiny test tables); `var user User` for all-zero values;
  `&T{...}` over `new(T)`.
- Map init: `make(map[K]V, hint)` for programmatic fill; literals for fixed content.
- Printf-style functions: end the name in `f` and declare format strings as `const`.
- Test tables: `tests` slice, `tt` case var, `give`/`want` field prefixes; split tables that need
  branching logic into separate test functions.
- Functional options for constructors with 3+ growing optional parameters.

## Lint baseline

Minimum recommended set, run via `golangci-lint`:

- `errcheck` — every error handled
- `goimports` — formatting + import grouping
- `revive` — style (successor of golint)
- `govet` — common mistakes
- `staticcheck` — static analysis

Consistency of enforcement matters more than the exact linter set.
