// Template: replace ExampleEntity* with project-specific names. Target
// locations:
//
//	internal/domain/valueobject/{entity}_{kind}_test.go
//	internal/domain/entity/{entity}_test.go
//
// Replace "app" in imports with the project's module path. Shown as one file;
// split to match the code under test.
//
// Domain tests take no context, no fakes, and no IO. A domain test that needs a
// fake is describing a usecase rule, not a domain rule.
package entity_test

import (
	"errors"
	"strings"
	"testing"

	"github.com/google/uuid"

	"app/internal/domain"
	"app/internal/domain/entity"
	"app/internal/domain/valueobject"
)

// --- Value object validation ------------------------------------------------

func TestNewExampleEntityName(t *testing.T) {
	t.Parallel()

	tests := []struct {
		name      string
		give      string
		wantValue string
		wantField string
	}{
		{name: "accepts a normal name", give: "quarterly report", wantValue: "quarterly report"},
		{name: "trims surrounding whitespace", give: "  report  ", wantValue: "report"},
		{name: "rejects empty", give: "", wantField: "name"},
		{name: "rejects whitespace only", give: "   ", wantField: "name"},
		{name: "rejects over the length limit", give: strings.Repeat("a", 101), wantField: "name"},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			t.Parallel()

			got, err := valueobject.NewExampleEntityName(tt.give)

			if tt.wantField == "" {
				if err != nil {
					t.Fatalf("NewExampleEntityName(%q) error = %v, want nil", tt.give, err)
				}
				if got.String() != tt.wantValue {
					t.Errorf("String() = %q, want %q", got.String(), tt.wantValue)
				}
				return
			}

			var validationErr *domain.ValidationError
			if !errors.As(err, &validationErr) {
				t.Fatalf("NewExampleEntityName(%q) error = %v, want *domain.ValidationError", tt.give, err)
			}
			if validationErr.Field != tt.wantField {
				t.Errorf("Field = %q, want %q", validationErr.Field, tt.wantField)
			}
		})
	}
}

// --- Entity invariants and transitions --------------------------------------

func TestExampleEntity_SoftDelete(t *testing.T) {
	t.Parallel()

	t.Run("marks the entity deleted and inactive", func(t *testing.T) {
		t.Parallel()

		e := newTestExampleEntity(t)

		if err := e.SoftDelete(); err != nil {
			t.Fatalf("SoftDelete() error = %v, want nil", err)
		}
		if !e.IsDeleted() {
			t.Error("IsDeleted() = false, want true")
		}
		if got := e.Status(); got != valueobject.ExampleEntityStatusInactive {
			t.Errorf("Status() = %v, want %v", got, valueobject.ExampleEntityStatusInactive)
		}
	})

	t.Run("deleting twice violates the invariant", func(t *testing.T) {
		t.Parallel()

		e := newTestExampleEntity(t)
		if err := e.SoftDelete(); err != nil {
			t.Fatalf("first SoftDelete() error = %v, want nil", err)
		}

		err := e.SoftDelete()

		var invariantErr *domain.InvariantError
		if !errors.As(err, &invariantErr) {
			t.Fatalf("second SoftDelete() error = %v, want *domain.InvariantError", err)
		}
	})
}

func TestExampleEntity_Rename(t *testing.T) {
	t.Parallel()

	t.Run("rejects renaming a deleted entity", func(t *testing.T) {
		t.Parallel()

		e := newTestExampleEntity(t)
		if err := e.SoftDelete(); err != nil {
			t.Fatalf("SoftDelete() error = %v, want nil", err)
		}

		err := e.Rename(mustExampleEntityName(t, "new name"))

		var invariantErr *domain.InvariantError
		if !errors.As(err, &invariantErr) {
			t.Fatalf("Rename() error = %v, want *domain.InvariantError", err)
		}
	})
}

// --- Fixtures ---------------------------------------------------------------

func newTestExampleEntity(t *testing.T) *entity.ExampleEntity {
	t.Helper()

	return entity.NewExampleEntity(
		uuid.New(),
		mustExampleEntityName(t, "quarterly report"),
		"https://example.test/a",
	)
}

func mustExampleEntityName(t *testing.T, raw string) valueobject.ExampleEntityName {
	t.Helper()

	name, err := valueobject.NewExampleEntityName(raw)
	if err != nil {
		t.Fatalf("NewExampleEntityName(%q) error = %v, want nil", raw, err)
	}

	return name
}
