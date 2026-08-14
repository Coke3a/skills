// Template: replace CreateExampleEntity*, ExampleEntity*, and examplefeature
// with project-specific names. Target location:
// internal/usecase/{feature}/{action}.go — package named after the feature.
// Replace "app" in imports with the project's module path.
// Usecases do not import Fiber, pgx, sqlc-generated packages, or handler DTOs.
//
// This usecase has no special opinion about any repository failure, so every
// error goes through usecase.ConvertError. When a usecase does have an opinion
// — an ErrNotFound that means "then create it", a unique violation on an
// idempotency key that means "already processed" — match it with errors.Is
// first and keep ConvertError as the fallback for the rest. Nothing raw leaves
// Execute, and a usecase logs only errors it handles itself.
package examplefeature

import (
	"context"
	"time"

	"github.com/google/uuid"

	"app/internal/domain/entity"
	"app/internal/domain/repository"
	"app/internal/domain/valueobject"
	"app/internal/usecase"
)

type CreateExampleEntityInput struct {
	OwnerID uuid.UUID
	Name    string
	URL     string
}

type CreateExampleEntityOutput struct {
	ID        uuid.UUID
	OwnerID   uuid.UUID
	Name      string
	URL       string
	Status    string
	CreatedAt time.Time
}

type CreateExampleEntityUsecase struct {
	exampleRepo repository.ExampleRepository
}

func NewCreateExampleEntityUsecase(exampleRepo repository.ExampleRepository) *CreateExampleEntityUsecase {
	return &CreateExampleEntityUsecase{exampleRepo: exampleRepo}
}

func (u *CreateExampleEntityUsecase) Execute(
	ctx context.Context,
	input CreateExampleEntityInput,
) (*CreateExampleEntityOutput, error) {
	name, err := valueobject.NewExampleEntityName(input.Name)
	if err != nil {
		return nil, usecase.ConvertError(err)
	}

	e := entity.NewExampleEntity(input.OwnerID, name, input.URL)

	if err := u.exampleRepo.Create(ctx, e); err != nil {
		return nil, usecase.ConvertError(err)
	}

	return &CreateExampleEntityOutput{
		ID:        e.ID().UUID(),
		OwnerID:   e.OwnerID(),
		Name:      e.Name().String(),
		URL:       e.URL(),
		Status:    e.Status().String(),
		CreatedAt: e.CreatedAt(),
	}, nil
}
