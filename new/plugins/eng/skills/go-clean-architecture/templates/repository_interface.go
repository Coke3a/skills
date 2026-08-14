// Template: replace ExampleRepository and ExampleEntity with project-specific
// names. Target location: internal/domain/repository/{entity}_repository.go
// Replace "app" in imports with the project's module path.
// Repository interfaces are persistence ports only: no pgx, sqlc-generated,
// pool, or transaction types here.
package repository

import (
	"context"

	"github.com/google/uuid"

	"app/internal/domain/entity"
	"app/internal/domain/valueobject"
)

// ExampleRepository is the persistence port for ExampleEntity.
// Implementations live in internal/infra/postgres and must return the
// sentinel errors defined in this package, wrapped with %w.
type ExampleRepository interface {
	Create(ctx context.Context, e *entity.ExampleEntity) error

	// FindByID returns a wrapped ErrNotFound when no row matches — never (nil, nil).
	FindByID(ctx context.Context, id valueobject.ExampleEntityID) (*entity.ExampleEntity, error)

	// ListByOwner returns an empty slice when nothing matches — no rows is a
	// valid result, not ErrNotFound.
	ListByOwner(ctx context.Context, ownerID uuid.UUID, limit, offset int32) ([]*entity.ExampleEntity, error)

	// Update returns a wrapped ErrNotFound when zero rows are affected.
	Update(ctx context.Context, e *entity.ExampleEntity) error

	// Delete returns a wrapped ErrNotFound when zero rows are affected.
	Delete(ctx context.Context, id valueobject.ExampleEntityID) error
}
