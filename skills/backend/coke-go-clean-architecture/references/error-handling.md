# Error Handling Reference

Use this reference for layer error responsibilities and conversion flow. The mechanics follow the
uber-go guide: sentinel errors for static matching, custom types for dynamic info, `%w` wrapping,
`errors.Is`/`errors.As` matching, and the handle-once principle.

## Flow

```text
domain error     -> usecase error -> API error
repository error -> usecase error -> API error
service error    -> usecase error -> API error
```

## Responsibilities

| Errors                  | Package                      | Responsibility                                           |
| ----------------------- | ---------------------------- | -------------------------------------------------------- |
| Domain errors           | `internal/domain`            | Validation failures and business invariant violations    |
| Repository errors       | `internal/domain/repository` | Persistence contract errors returned by repository ports |
| Service errors          | `internal/domain/service`    | External-service contract errors returned by IO ports    |
| Usecase errors          | `internal/usecase`           | User-facing application semantics                        |
| API error handler       | `internal/handler/apierror`  | HTTP status and response body mapping                    |

## Layer ownership

One rule answers both "who converts?" and "who logs?": **the layer with enough context to decide an
error's fate owns it, and whoever decides is the one who logs.**

| Layer              | Error responsibility                                                                           | Logging                        |
| ------------------ | ---------------------------------------------------------------------------------------------- | ------------------------------ |
| `internal/infra`   | Translate driver/provider errors into the domain sentinel vocabulary, wrap with `%w`, return    | Never                          |
| `internal/usecase` | Owns error policy: decides which errors end here (fallback, degrade, skip) and which travel up   | Only errors it handles itself  |
| `internal/handler` | Returns errors untouched                                                                         | Edge logs once, everything that reaches it |

Why infra never logs: a repository cannot know whether a failure is a problem. `FindByID` returning
`ErrNotFound` is a 404 in one usecase and the normal "then create it" path in another — logging there
turns a correct code path into error noise, and the edge will log it again if it really was a
failure. Infra does not own the vocabulary it returns either: the sentinels live in
`internal/domain/repository` and `internal/domain/service` precisely so usecases can match on them
without importing infra.

### Usecase: wrap for context, do not log

Business context belongs in the error chain, not in an extra log line. Wrapping keeps handle-once
intact and still gives the edge a single log entry with the whole story in it.

```go
if err := u.paymentSvc.Charge(ctx, sub.CustomerID(), amount); err != nil {
	return nil, fmt.Errorf("renew_subscription: customer %s: %w", sub.CustomerID(), usecase.ConvertError(err))
}
```

### Usecase: log only what it swallows

The one place `slog` belongs in a usecase is a path the usecase deliberately degrades or skips —
there the error stops, so logging it is the handling.

```go
if err := u.mailer.SendWelcome(ctx, user.Email()); err != nil {
	// Signup already succeeded; a failed welcome mail must not fail the request.
	slog.ErrorContext(ctx, "welcome email failed", "user_id", user.ID(), "err", err)
}
```

`log/slog` is stdlib, so using it directly does not violate the dependency rule and no `Logger` port
is needed. But `slog` on most usecase paths means errors are being logged *and* returned: delete the
log line and let the edge do its job.

## Domain errors

```go
// internal/domain/errors.go
package domain

import (
	"errors"
	"fmt"
)

var (
	ErrNotFound = errors.New("not found")
	ErrConflict = errors.New("conflict")
)

// ValidationError reports an invalid user-provided field.
type ValidationError struct {
	Field  string
	Reason string
}

func (e *ValidationError) Error() string {
	return fmt.Sprintf("invalid field %q: %s", e.Field, e.Reason)
}

// InvariantError reports a violated business invariant.
type InvariantError struct {
	Reason string
}

func (e *InvariantError) Error() string {
	return fmt.Sprintf("invariant violation: %s", e.Reason)
}
```

## Repository errors

Repository interfaces promise these sentinels so infra details do not leak upward directly.
Implementations wrap driver errors with `%w` and an `entity.operation` prefix.

```go
// internal/domain/repository/errors.go
package repository

import "errors"

var (
	ErrNotFound            = errors.New("entity not found")
	ErrUniqueViolation     = errors.New("unique constraint violation")
	ErrForeignKeyViolation = errors.New("foreign key violation")
)
```

Contract rules:

- `FindBy...` methods return `ErrNotFound` (wrapped) when no row matches — never `(nil, nil)`.
- Updates/deletes that expect an existing row return `ErrNotFound` when zero rows are affected.
- List queries return an empty slice, not `ErrNotFound` — no rows is a valid result, not a failure.
- Driver errors that match no sentinel are wrapped as-is: `fmt.Errorf("example_entity.create: %w", err)`.
- A repository returns errors; it never logs them, retries, substitutes a default, or decides what a
  failure means for the caller (see `references/repository-sqlc.md`).

## Service errors

External-service ports promise their own sentinels for the same reason repositories do: the usecase
must be able to match on a failure without importing the provider client.

```go
// internal/domain/service/errors.go
package service

import "errors"

var (
	ErrNotFound    = errors.New("remote resource not found")
	ErrRejected    = errors.New("request rejected by provider")
	ErrUnavailable = errors.New("service unavailable")
	ErrTimeout     = errors.New("service timeout")
)
```

Contract rules:

- `ErrRejected` is a business outcome the caller may act on or show (declined card, invalid recipient).
- `ErrUnavailable` and `ErrTimeout` are operational — the usecase decides whether to degrade, retry,
  or fail the request.
- Provider SDK error types, HTTP status codes, and response bodies stay inside the infra adapter.
  Wrap them behind a sentinel with the detail preserved in the chain:
  `fmt.Errorf("payment.charge: %s: %w", providerCode, service.ErrRejected)`.

## Usecase errors

Usecases turn domain, repository, and service failures into user-facing application semantics.

```go
// internal/usecase/errors.go
package usecase

import (
	"errors"
	"fmt"

	"app/internal/domain"
	"app/internal/domain/repository"
	"app/internal/domain/service"
)

var (
	ErrNotFound    = errors.New("not found")
	ErrValidation  = errors.New("validation failed")
	ErrConflict    = errors.New("conflict")
	ErrRejected    = errors.New("rejected")
	ErrUnavailable = errors.New("dependency unavailable")
	ErrInternal    = errors.New("internal error")
)

// ConvertError is the fallback mapping for domain, repository, and service
// errors a usecase has no specific opinion about. It preserves the original
// chain so the edge logs the full story.
func ConvertError(err error) error {
	var validationErr *domain.ValidationError
	var invariantErr *domain.InvariantError

	switch {
	case errors.As(err, &validationErr), errors.As(err, &invariantErr):
		return fmt.Errorf("%w: %s", ErrValidation, err)
	case errors.Is(err, domain.ErrNotFound),
		errors.Is(err, repository.ErrNotFound),
		errors.Is(err, service.ErrNotFound):
		return fmt.Errorf("%w: %s", ErrNotFound, err)
	case errors.Is(err, domain.ErrConflict),
		errors.Is(err, repository.ErrUniqueViolation),
		errors.Is(err, repository.ErrForeignKeyViolation):
		return fmt.Errorf("%w: %s", ErrConflict, err)
	case errors.Is(err, service.ErrRejected):
		return fmt.Errorf("%w: %s", ErrRejected, err)
	case errors.Is(err, service.ErrUnavailable), errors.Is(err, service.ErrTimeout):
		return fmt.Errorf("%w: %s", ErrUnavailable, err)
	default:
		return fmt.Errorf("%w: %s", ErrInternal, err)
	}
}
```

### ConvertError is a fallback, not a policy

`ConvertError` exists so no raw error escapes a usecase as a silent 500. It is not a substitute for
the usecase deciding semantics — calling it blindly moves the decision out of the layer that owns it.
The same sentinel means different things in different usecases:

- `repository.ErrUniqueViolation` on "create a user with an existing email" → conflict, which is what
  `ConvertError` produces.
- `repository.ErrUniqueViolation` on an idempotency-key insert → the request was already processed;
  return the existing result. Not an error at all.

Match what the usecase has an opinion about first, then fall back:

```go
existing, err := u.repo.FindByIdempotencyKey(ctx, key)
switch {
case err == nil:
	return toOutput(existing), nil // this usecase's own decision: not an error
case errors.Is(err, repository.ErrNotFound):
	// expected — continue with the normal create path
default:
	return nil, usecase.ConvertError(err)
}
```

A usecase that needs a specific user-facing message wraps the sentinel itself:
`fmt.Errorf("%w: example entity %s", ErrNotFound, id)`. Prefer that over `ConvertError` whenever the
wrapped chain could carry provider or driver detail into a response body.

## API error mapping

The centralized Fiber `ErrorHandler` is the single place errors become HTTP responses.

| Usecase error    | HTTP status | Error code            |
| ---------------- | ----------- | --------------------- |
| `ErrNotFound`    | 404         | `NOT_FOUND`           |
| `ErrValidation`  | 400         | `VALIDATION_ERROR`    |
| `ErrConflict`    | 409         | `CONFLICT`            |
| `ErrRejected`    | 422         | `REJECTED`            |
| `ErrUnavailable` | 503         | `SERVICE_UNAVAILABLE` |
| `ErrInternal`    | 500         | `INTERNAL_ERROR`      |

```go
// internal/handler/apierror/apierror.go
package apierror

import (
	"errors"
	"log/slog"

	"github.com/gofiber/fiber/v3"

	"app/internal/usecase"
)

type response struct {
	Code    string `json:"code"`
	Message string `json:"message"`
}

// Handler is wired into fiber.Config.ErrorHandler and owns all error → HTTP mapping.
// It is also the single logging point for every error that reaches the edge.
func Handler(c fiber.Ctx, err error) error {
	var fiberErr *fiber.Error
	if errors.As(err, &fiberErr) {
		return c.Status(fiberErr.Code).JSON(response{Code: "REQUEST_ERROR", Message: fiberErr.Message})
	}

	switch {
	case errors.Is(err, usecase.ErrNotFound):
		return c.Status(fiber.StatusNotFound).JSON(response{Code: "NOT_FOUND", Message: err.Error()})
	case errors.Is(err, usecase.ErrValidation):
		return c.Status(fiber.StatusBadRequest).JSON(response{Code: "VALIDATION_ERROR", Message: err.Error()})
	case errors.Is(err, usecase.ErrConflict):
		return c.Status(fiber.StatusConflict).JSON(response{Code: "CONFLICT", Message: err.Error()})
	case errors.Is(err, usecase.ErrRejected):
		return c.Status(fiber.StatusUnprocessableEntity).JSON(response{Code: "REJECTED", Message: err.Error()})
	case errors.Is(err, usecase.ErrUnavailable):
		// Operational failure: log the chain, tell the client nothing about the dependency.
		// fiber.Ctx implements context.Context in v3, so it is passed straight to slog.
		slog.ErrorContext(c, "dependency unavailable", "err", err, "path", c.Path())
		return c.Status(fiber.StatusServiceUnavailable).JSON(response{Code: "SERVICE_UNAVAILABLE", Message: "service unavailable"})
	default:
		slog.ErrorContext(c, "unhandled error", "err", err, "path", c.Path())
		return c.Status(fiber.StatusInternalServerError).JSON(response{Code: "INTERNAL_ERROR", Message: "internal error"})
	}
}
```

A raw `domain`, `repository`, or `service` error reaching this handler falls through to the `default`
branch and becomes a silent 500. That is the failure mode the "every error leaving `Execute` is a
usecase sentinel" rule exists to prevent — and the reason handlers must never match on
`repository.Err*` to paper over it.

## Rules

- Log internal error chains server-side; return a generic message for internal failures.
- Do not expose database, pool, SQL, or infrastructure details in HTTP responses.
- Handle each error once: wrap and return, or log and degrade — never log *and* return.
- `internal/infra` never logs: it wraps and returns. A usecase logs only errors it handles itself.
  Everything else is logged once at the edge (`apierror.Handler`, worker loop).
- Every error leaving a usecase `Execute` is a usecase sentinel. A raw domain, repository, or service
  error reaching a handler is a bug that surfaces as a silent 500.
- Handlers never match on `domain.Err*`, `repository.Err*`, or `service.Err*` — only the central
  `apierror.Handler` matches, and only on `usecase.Err*`.
- Wrap with `%w` when callers may need `errors.Is`/`errors.As`; the wrapped sentinel becomes part
  of the layer contract.
- Keep wrap context succinct — `"example_entity.create: %w"`, not `"failed to create ...: %w"`;
  the logging layer adds the failure framing.
- Sentinels use the `Err` prefix; custom error types use the `Error` suffix (uber-go naming).
- No `panic` in production code paths. Panic only for programmer errors during startup
  (`template.Must`-style); recover middleware is a safety net, not a control-flow tool.
- `os.Exit`/`log.Fatal` only in `main()`, preferably once: `if err := run(); err != nil { log.Fatal(err) }`.
