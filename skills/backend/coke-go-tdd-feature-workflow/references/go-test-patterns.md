# Go Test Patterns

Assumes Go 1.22+, so no `tt := tt` loop-variable copy is needed.

## Table-driven with give/want

The uber-go naming this architecture already follows: the slice is `tests`, the loop variable is
`tt`, inputs are `give*`, expectations are `want*`.

```go
tests := []struct {
	name     string
	giveName string
	wantErr  error
}{
	{name: "empty name is rejected", giveName: "", wantErr: domain.ErrValidation},
	{name: "name at max length is accepted", giveName: strings.Repeat("a", 120)},
}

for _, tt := range tests {
	t.Run(tt.name, func(t *testing.T) {
		t.Parallel()
		// ...
	})
}
```

Use a table when the cases differ only in data. Two cases that need different setup are two
functions, not two rows with a `setup func()` field — a table with a callback column has stopped
being a table.

## Parallelism

`t.Parallel()` at the top of every test and every subtest, by default. It surfaces shared state,
and combined with `-race` it is the only thing that catches the concurrency bugs the worker and pool
patterns in `coke-go-clean-architecture` can produce.

A test cannot be parallel if it mutates a package-level variable, sets an environment variable, or
shares a fake with a sibling. All three are worth removing rather than working around.

## Assertions

Standard library, no assertion framework:

```go
if err != nil {
	t.Fatalf("Execute() error = %v, want nil", err)
}
if got.Name != tt.wantName {
	t.Errorf("Name = %q, want %q", got.Name, tt.wantName)
}
```

- `t.Fatalf` when continuing would panic or produce noise; `t.Errorf` when the test can keep
  checking and report several problems at once.
- Message format is `<what> = <got>, want <want>`. That is the convention the standard library uses
  and it makes a failure readable without opening the file.
- Structs compare with `cmp.Diff` from `github.com/google/go-cmp` — a test-only dependency and the
  de-facto standard:

```go
if diff := cmp.Diff(want, got, cmpopts.IgnoreFields(Output{}, "CreatedAt")); diff != "" {
	t.Errorf("Execute() mismatch (-want +got):\n%s", diff)
}
```

## Error assertions

Sentinels use `errors.Is`, typed errors use `errors.As`. Never compare error strings.

```go
if !errors.Is(err, usecase.ErrConflict) {
	t.Fatalf("Execute() error = %v, want errors.Is(err, usecase.ErrConflict)", err)
}
```

```go
var validationErr *domain.ValidationError
if !errors.As(err, &validationErr) {
	t.Fatalf("error = %v, want *domain.ValidationError", err)
}
if validationErr.Field != "name" {
	t.Errorf("Field = %q, want %q", validationErr.Field, "name")
}
```

### The leak assertion

Usecase error tests assert in both directions — the right sentinel is present, and the lower-layer
sentinel is gone:

```go
if errors.Is(err, repository.ErrUniqueViolation) {
	t.Errorf("repository sentinel escaped Execute(): %v", err)
}
```

This works because `usecase.ConvertError` joins the original with `%s`, not `%w`. The chain is
preserved as text for the edge to log, but it is no longer matchable — so a handler that started
switching on `repository.Err*` would be broken, and a usecase that forgot to convert is caught here
instead of becoming a silent `500` in `apierror.Handler`'s default branch.

## Context

Every IO-taking function takes `context.Context` first. In tests:

- `context.Background()` for the ordinary case.
- `t.Context()` when the test should cancel on completion (Go 1.24+); otherwise
  `ctx, cancel := context.WithCancel(context.Background())` with `t.Cleanup(cancel)`.
- To prove cancellation propagates, cancel before the call and assert `context.Canceled` comes back.

## Cleanup and helpers

`t.Cleanup(fn)` over `defer` — it runs after parallel subtests finish, which `defer` in the parent
does not.

`t.Helper()` as the first line of every helper, so failures point at the caller.

## Fiber handler tests

```go
app := fiber.New(fiber.Config{ErrorHandler: apierror.Handler})
publicapi.NewExampleEntityHandler(uc).Register(app)

req := httptest.NewRequest(http.MethodPost, "/example-entities", strings.NewReader(body))
req.Header.Set("Content-Type", "application/json")

resp, err := app.Test(req)
```

- Use the project's **real** `ErrorHandler`. A test app with the default handler proves nothing about
  status-code mapping, which is the main reason to write a handler test at all.
- `app.Test` defaults to a 1-second timeout. `fiber.TestConfig{Timeout: 0}` disables it for unit
  tests; raise it only if the handler does real IO, which at this level it should not.
- Register through the handler's own `Register(r fiber.Router)` so route wiring is under test rather
  than restated in the test.
- `defer resp.Body.Close()`, and decode into a struct rather than asserting on the raw string.

## Time

A usecase that reads `time.Now()` directly cannot be tested for expiry, TTL, or scheduling. Inject a
clock through the constructor — the same way repositories are injected — and give the fake a fixed
instant. Do not add a package-level `var now = time.Now` for tests to swap; that is shared mutable
state and it forbids `t.Parallel()`.

## Running

```sh
go test -race ./internal/usecase/examplefeature/   # the narrowest package, during the loop
go test -race ./...                                # before calling it done
```

`-race` is not optional. Add `-count=1` when a cached pass is suspicious, and `-run TestName/subtest`
to isolate one case.
