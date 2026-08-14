// Template: three value object shapes — ID type, validated value object, and
// status enum. Split into separate files in a real project:
//
//	internal/domain/valueobject/example_entity_id.go
//	internal/domain/valueobject/example_entity_name.go
//	internal/domain/valueobject/example_entity_status.go
//
// Replace "app" in imports with the project's module path.
package valueobject

import (
	"fmt"
	"strings"
	"unicode/utf8"

	"github.com/google/uuid"

	"app/internal/domain"
)

// --- ID value object -------------------------------------------------------
// Struct-wrapped so a raw uuid.UUID cannot be passed where an entity ID is
// expected, and IDs of different entities cannot be mixed up.

type ExampleEntityID struct {
	value uuid.UUID
}

func NewExampleEntityID() ExampleEntityID {
	return ExampleEntityID{value: uuid.New()}
}

// ParseExampleEntityID validates external string input such as a path param.
func ParseExampleEntityID(raw string) (ExampleEntityID, error) {
	id, err := uuid.Parse(raw)
	if err != nil {
		return ExampleEntityID{}, &domain.ValidationError{Field: "id", Reason: "must be a valid UUID"}
	}

	return ExampleEntityID{value: id}, nil
}

// ExampleEntityIDFromTrusted wraps a UUID that is already trusted, such as a
// value read back from the database.
func ExampleEntityIDFromTrusted(id uuid.UUID) ExampleEntityID {
	return ExampleEntityID{value: id}
}

func (id ExampleEntityID) UUID() uuid.UUID {
	return id.value
}

func (id ExampleEntityID) String() string {
	return id.value.String()
}

// --- Validated value object ------------------------------------------------

const maxExampleEntityNameLength = 100

type ExampleEntityName struct {
	value string
}

func NewExampleEntityName(raw string) (ExampleEntityName, error) {
	trimmed := strings.TrimSpace(raw)
	if trimmed == "" {
		return ExampleEntityName{}, &domain.ValidationError{Field: "name", Reason: "must not be empty"}
	}
	if utf8.RuneCountInString(trimmed) > maxExampleEntityNameLength {
		return ExampleEntityName{}, &domain.ValidationError{
			Field:  "name",
			Reason: fmt.Sprintf("must be at most %d characters", maxExampleEntityNameLength),
		}
	}

	return ExampleEntityName{value: trimmed}, nil
}

// ExampleEntityNameFromTrusted skips validation for database reconstruction.
func ExampleEntityNameFromTrusted(value string) ExampleEntityName {
	return ExampleEntityName{value: value}
}

func (n ExampleEntityName) String() string {
	return n.value
}

// --- Status enum -----------------------------------------------------------
// String-typed so database and JSON mapping stay explicit.

type ExampleEntityStatus string

const (
	ExampleEntityStatusActive   ExampleEntityStatus = "active"
	ExampleEntityStatusInactive ExampleEntityStatus = "inactive"
)

// ParseExampleEntityStatus validates external input, including status columns
// read from the database — an unknown persisted value is a real error.
func ParseExampleEntityStatus(raw string) (ExampleEntityStatus, error) {
	status := ExampleEntityStatus(raw)
	switch status {
	case ExampleEntityStatusActive, ExampleEntityStatusInactive:
		return status, nil
	default:
		return "", &domain.ValidationError{Field: "status", Reason: fmt.Sprintf("unknown status %q", raw)}
	}
}

func (s ExampleEntityStatus) String() string {
	return string(s)
}
