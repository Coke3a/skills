// Template: replace ExampleRepository, ExampleEntity, and examplefeature with
// project-specific names. Target location:
// internal/usecase/{feature}/fake_test.go
// Replace "app" in imports with the project's module path.
//
// Hand-written fake, no mocking library. Two halves: stubbed results that drive
// the branch under test, and recorded calls that let a test assert a side
// effect happened. The zero value is the happy path, so each test sets only the
// field its case is about.
//
// Promote this to internal/domain/repository/repotest/ only when a second
// package needs it — usually when the handler test wants the same fake.
package examplefeature_test

import (
	"context"
	"fmt"

	"github.com/google/uuid"

	"app/internal/domain/entity"
	"app/internal/domain/repository"
	"app/internal/domain/valueobject"
)

// Compile-time check. The fake is bound by the same port as the real
// implementation, so a method added to the interface breaks this file at build
// time instead of surfacing later as a confusing assertion failure.
var _ repository.ExampleRepository = (*fakeExampleRepository)(nil)

type fakeExampleRepository struct {
	// Stubbed results.
	createErr  error
	findResult *entity.ExampleEntity
	findErr    error
	listResult []*entity.ExampleEntity
	listErr    error
	updateErr  error
	deleteErr  error

	// Recorded calls.
	created  []*entity.ExampleEntity
	updated  []*entity.ExampleEntity
	deleted  []valueobject.ExampleEntityID
	findByID []valueobject.ExampleEntityID
}

func (f *fakeExampleRepository) Create(_ context.Context, e *entity.ExampleEntity) error {
	if f.createErr != nil {
		return f.createErr
	}

	f.created = append(f.created, e)
	return nil
}

func (f *fakeExampleRepository) FindByID(
	_ context.Context,
	id valueobject.ExampleEntityID,
) (*entity.ExampleEntity, error) {
	f.findByID = append(f.findByID, id)
	if f.findErr != nil {
		return nil, f.findErr
	}

	return f.findResult, nil
}

func (f *fakeExampleRepository) ListByOwner(
	_ context.Context,
	_ uuid.UUID,
	_, _ int32,
) ([]*entity.ExampleEntity, error) {
	if f.listErr != nil {
		return nil, f.listErr
	}

	// Mirrors the port contract: no rows is an empty slice, not ErrNotFound.
	if f.listResult == nil {
		return []*entity.ExampleEntity{}, nil
	}

	return f.listResult, nil
}

func (f *fakeExampleRepository) Update(_ context.Context, e *entity.ExampleEntity) error {
	if f.updateErr != nil {
		return f.updateErr
	}

	f.updated = append(f.updated, e)
	return nil
}

func (f *fakeExampleRepository) Delete(_ context.Context, id valueobject.ExampleEntityID) error {
	if f.deleteErr != nil {
		return f.deleteErr
	}

	f.deleted = append(f.deleted, id)
	return nil
}

// repoError builds an error shaped the way internal/infra/postgres builds it:
// operation context wrapped around a sentinel with %w. A fake returning a bare
// sentinel exercises a chain production never produces.
func repoError(op string, sentinel error) error {
	return fmt.Errorf("example_entity.%s: %w", op, sentinel)
}
