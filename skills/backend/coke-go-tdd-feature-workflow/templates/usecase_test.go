// Template: replace CreateExampleEntity*, ExampleEntity*, and examplefeature
// with project-specific names. Target location:
// internal/usecase/{feature}/{action}_test.go
// Replace "app" in imports with the project's module path.
//
// This file is written BEFORE internal/usecase/{feature}/{action}.go exists.
// Whatever fakeExampleRepository must provide to satisfy these tests is what
// repository.ExampleRepository must declare — the port is discovered here, not
// derived from the database schema.
//
// Pairs with templates/fake_repository.go in the same package.
package examplefeature_test

import (
	"context"
	"errors"
	"testing"

	"github.com/google/uuid"

	"app/internal/domain/repository"
	"app/internal/usecase"
	"app/internal/usecase/examplefeature"
)

func TestCreateExampleEntity_Success(t *testing.T) {
	t.Parallel()

	repo := &fakeExampleRepository{}
	uc := examplefeature.NewCreateExampleEntityUsecase(repo)

	got, err := uc.Execute(context.Background(), examplefeature.CreateExampleEntityInput{
		OwnerID: uuid.New(),
		Name:    "quarterly report",
		URL:     "https://example.test/a",
	})
	if err != nil {
		t.Fatalf("Execute() error = %v, want nil", err)
	}

	if got.Name != "quarterly report" {
		t.Errorf("Name = %q, want %q", got.Name, "quarterly report")
	}
	if got.Status != "active" {
		t.Errorf("Status = %q, want %q", got.Status, "active")
	}

	// The side effect is part of the behaviour: the entity reached the port.
	if len(repo.created) != 1 {
		t.Fatalf("repository received %d entities, want 1", len(repo.created))
	}
	if persisted := repo.created[0].Name().String(); persisted != "quarterly report" {
		t.Errorf("persisted name = %q, want %q", persisted, "quarterly report")
	}
}

func TestCreateExampleEntity_Validation(t *testing.T) {
	t.Parallel()

	tests := []struct {
		name     string
		giveName string
	}{
		{name: "empty name", giveName: ""},
		{name: "whitespace only", giveName: "   "},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			t.Parallel()

			repo := &fakeExampleRepository{}
			uc := examplefeature.NewCreateExampleEntityUsecase(repo)

			_, err := uc.Execute(context.Background(), examplefeature.CreateExampleEntityInput{
				OwnerID: uuid.New(),
				Name:    tt.giveName,
				URL:     "https://example.test/a",
			})
			if !errors.Is(err, usecase.ErrValidation) {
				t.Fatalf("Execute() error = %v, want errors.Is(err, usecase.ErrValidation)", err)
			}
			if len(repo.created) != 0 {
				t.Errorf("repository called %d times, want 0 — validation must short-circuit", len(repo.created))
			}
		})
	}
}

// TestCreateExampleEntity_ErrorMapping is the test that enforces the error
// ownership rules from coke-go-clean-architecture. Each case asserts both
// directions: the usecase sentinel the caller must see, and the lower-layer
// sentinel that must no longer be matchable.
//
// The leak assertion works because usecase.ConvertError joins the original with
// %s rather than %w — the chain survives as text for the edge to log, but stops
// being something a handler could switch on.
func TestCreateExampleEntity_ErrorMapping(t *testing.T) {
	t.Parallel()

	tests := []struct {
		name        string
		giveRepoErr error
		wantErr     error
		wantNoLeak  error
	}{
		{
			name:        "unique violation becomes conflict",
			giveRepoErr: repoError("create", repository.ErrUniqueViolation),
			wantErr:     usecase.ErrConflict,
			wantNoLeak:  repository.ErrUniqueViolation,
		},
		{
			name:        "foreign key violation becomes conflict",
			giveRepoErr: repoError("create", repository.ErrForeignKeyViolation),
			wantErr:     usecase.ErrConflict,
			wantNoLeak:  repository.ErrForeignKeyViolation,
		},
		{
			name:        "unrecognised driver error becomes internal",
			giveRepoErr: repoError("create", errors.New("connection reset by peer")),
			wantErr:     usecase.ErrInternal,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			t.Parallel()

			repo := &fakeExampleRepository{createErr: tt.giveRepoErr}
			uc := examplefeature.NewCreateExampleEntityUsecase(repo)

			_, err := uc.Execute(context.Background(), examplefeature.CreateExampleEntityInput{
				OwnerID: uuid.New(),
				Name:    "quarterly report",
				URL:     "https://example.test/a",
			})
			if !errors.Is(err, tt.wantErr) {
				t.Fatalf("Execute() error = %v, want errors.Is(err, %v)", err, tt.wantErr)
			}
			if tt.wantNoLeak != nil && errors.Is(err, tt.wantNoLeak) {
				t.Errorf("repository sentinel escaped Execute(): %v", err)
			}
		})
	}
}
