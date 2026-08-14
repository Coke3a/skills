// Template: layered error values and conversion helpers. This file shows all
// layers together; in a real project split the sections into:
//   internal/domain/errors.go
//   internal/domain/repository/errors.go
//   internal/domain/service/errors.go
//   internal/usecase/errors.go
//   internal/handler/apierror/apierror.go
//
// Ownership: infra maps driver/provider errors onto the domain sentinels below
// and returns them without logging. The usecase decides which errors stop there
// and which travel up. The edge handler logs once and maps to HTTP.
// Replace "app" in imports with the project's module path.

// --- internal/domain/errors.go ---------------------------------------------
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

// --- internal/domain/repository/errors.go ----------------------------------
// package repository
//
// var (
// 	ErrNotFound            = errors.New("entity not found")
// 	ErrUniqueViolation     = errors.New("unique constraint violation")
// 	ErrForeignKeyViolation = errors.New("foreign key violation")
// )

// --- internal/domain/service/errors.go --------------------------------------
// package service
//
// var (
// 	ErrNotFound    = errors.New("remote resource not found")
// 	ErrRejected    = errors.New("request rejected by provider")
// 	ErrUnavailable = errors.New("service unavailable")
// 	ErrTimeout     = errors.New("service timeout")
// )

// --- internal/usecase/errors.go --------------------------------------------
// package usecase
//
// import (
// 	"errors"
// 	"fmt"
//
// 	"app/internal/domain"
// 	"app/internal/domain/repository"
// 	"app/internal/domain/service"
// )
//
// var (
// 	ErrNotFound    = errors.New("not found")
// 	ErrValidation  = errors.New("validation failed")
// 	ErrConflict    = errors.New("conflict")
// 	ErrRejected    = errors.New("rejected")
// 	ErrUnavailable = errors.New("dependency unavailable")
// 	ErrInternal    = errors.New("internal error")
// )
//
// // ConvertError is the fallback mapping for domain, repository, and service
// // errors this usecase has no specific opinion about. Errors the usecase does
// // have an opinion about are matched with errors.Is/errors.As before this is
// // reached. The original chain is preserved so the edge logs the full story.
// func ConvertError(err error) error {
// 	var validationErr *domain.ValidationError
// 	var invariantErr *domain.InvariantError
//
// 	switch {
// 	case errors.As(err, &validationErr), errors.As(err, &invariantErr):
// 		return fmt.Errorf("%w: %s", ErrValidation, err)
// 	case errors.Is(err, domain.ErrNotFound),
// 		errors.Is(err, repository.ErrNotFound),
// 		errors.Is(err, service.ErrNotFound):
// 		return fmt.Errorf("%w: %s", ErrNotFound, err)
// 	case errors.Is(err, domain.ErrConflict),
// 		errors.Is(err, repository.ErrUniqueViolation),
// 		errors.Is(err, repository.ErrForeignKeyViolation):
// 		return fmt.Errorf("%w: %s", ErrConflict, err)
// 	case errors.Is(err, service.ErrRejected):
// 		return fmt.Errorf("%w: %s", ErrRejected, err)
// 	case errors.Is(err, service.ErrUnavailable), errors.Is(err, service.ErrTimeout):
// 		return fmt.Errorf("%w: %s", ErrUnavailable, err)
// 	default:
// 		return fmt.Errorf("%w: %s", ErrInternal, err)
// 	}
// }

// --- internal/handler/apierror/apierror.go ---------------------------------
// package apierror
//
// import (
// 	"errors"
// 	"log/slog"
//
// 	"github.com/gofiber/fiber/v3"
//
// 	"app/internal/usecase"
// )
//
// type response struct {
// 	Code    string `json:"code"`
// 	Message string `json:"message"`
// }
//
// // Handler is wired into fiber.Config.ErrorHandler and owns all
// // error → HTTP mapping. It is also the single logging point for every error
// // that reaches the edge. Internal chains are logged, never exposed.
// //
// // A raw domain/repository/service error lands in the default branch and
// // becomes a silent 500 — that is the bug the "Execute returns only usecase
// // sentinels" rule prevents. Never add repository.Err* cases here.
// func Handler(c fiber.Ctx, err error) error {
// 	var fiberErr *fiber.Error
// 	if errors.As(err, &fiberErr) {
// 		return c.Status(fiberErr.Code).JSON(response{Code: "REQUEST_ERROR", Message: fiberErr.Message})
// 	}
//
// 	switch {
// 	case errors.Is(err, usecase.ErrNotFound):
// 		return c.Status(fiber.StatusNotFound).JSON(response{Code: "NOT_FOUND", Message: err.Error()})
// 	case errors.Is(err, usecase.ErrValidation):
// 		return c.Status(fiber.StatusBadRequest).JSON(response{Code: "VALIDATION_ERROR", Message: err.Error()})
// 	case errors.Is(err, usecase.ErrConflict):
// 		return c.Status(fiber.StatusConflict).JSON(response{Code: "CONFLICT", Message: err.Error()})
// 	case errors.Is(err, usecase.ErrRejected):
// 		return c.Status(fiber.StatusUnprocessableEntity).JSON(response{Code: "REJECTED", Message: err.Error()})
// 	case errors.Is(err, usecase.ErrUnavailable):
// 		// fiber.Ctx implements context.Context in v3, so it goes straight to slog.
// 		slog.ErrorContext(c, "dependency unavailable", "err", err, "path", c.Path())
// 		return c.Status(fiber.StatusServiceUnavailable).JSON(response{Code: "SERVICE_UNAVAILABLE", Message: "service unavailable"})
// 	default:
// 		slog.ErrorContext(c, "unhandled error", "err", err, "path", c.Path())
// 		return c.Status(fiber.StatusInternalServerError).JSON(response{Code: "INTERNAL_ERROR", Message: "internal error"})
// 	}
// }
