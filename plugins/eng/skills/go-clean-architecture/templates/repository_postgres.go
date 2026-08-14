// Template: replace ExampleRepository, ExampleEntity, and query names with
// project-specific names. Target location:
// internal/infra/postgres/{entity}_repository.go
// Replace "app" in imports with the project's module path.
// sqlc-generated types (package gen) must not leak past this package.
// This layer maps and returns errors only: no logging, no retry, no fallback,
// no business decisions — the usecase owns all of those.
package postgres

import (
	"context"
	"errors"
	"fmt"
	"time"

	"github.com/jackc/pgx/v5"
	"github.com/jackc/pgx/v5/pgconn"
	"github.com/jackc/pgx/v5/pgxpool"

	"app/internal/domain/entity"
	"app/internal/domain/repository"
	"app/internal/domain/valueobject"
	"app/internal/infra/postgres/gen"
)

// Compile-time check that the implementation satisfies the domain port.
var _ repository.ExampleRepository = (*ExampleRepository)(nil)

type ExampleRepository struct {
	q *gen.Queries
}

func NewExampleRepository(pool *pgxpool.Pool) *ExampleRepository {
	return &ExampleRepository{q: gen.New(pool)}
}

func (r *ExampleRepository) Create(ctx context.Context, e *entity.ExampleEntity) error {
	err := r.q.CreateExampleEntity(ctx, gen.CreateExampleEntityParams{
		ID:        e.ID().UUID(),
		OwnerID:   e.OwnerID(),
		Name:      e.Name().String(),
		Url:       e.URL(),
		Status:    e.Status().String(),
		CreatedAt: e.CreatedAt(),
		UpdatedAt: e.UpdatedAt(),
	})
	if err != nil {
		return mapPgError("example_entity.create", err)
	}

	return nil
}

func (r *ExampleRepository) FindByID(
	ctx context.Context,
	id valueobject.ExampleEntityID,
) (*entity.ExampleEntity, error) {
	row, err := r.q.GetExampleEntityByID(ctx, id.UUID())
	if err != nil {
		return nil, mapPgError("example_entity.find_by_id", err)
	}

	return rowToExampleEntity(row)
}

func (r *ExampleRepository) Update(ctx context.Context, e *entity.ExampleEntity) error {
	affected, err := r.q.UpdateExampleEntity(ctx, gen.UpdateExampleEntityParams{
		ID:        e.ID().UUID(),
		Name:      e.Name().String(),
		Url:       e.URL(),
		Status:    e.Status().String(),
		UpdatedAt: e.UpdatedAt(),
	})
	if err != nil {
		return mapPgError("example_entity.update", err)
	}
	if affected == 0 {
		return fmt.Errorf("example_entity.update: id %s: %w", e.ID(), repository.ErrNotFound)
	}

	return nil
}

func (r *ExampleRepository) Delete(ctx context.Context, id valueobject.ExampleEntityID) error {
	affected, err := r.q.DeleteExampleEntity(ctx, id.UUID())
	if err != nil {
		return mapPgError("example_entity.delete", err)
	}
	if affected == 0 {
		return fmt.Errorf("example_entity.delete: id %s: %w", id, repository.ErrNotFound)
	}

	return nil
}

// rowToExampleEntity converts a sqlc-generated row into a domain entity.
// Persisted values are trusted except enums, which are parsed so corrupt data
// surfaces as an error instead of an invalid entity.
func rowToExampleEntity(row gen.ExampleEntity) (*entity.ExampleEntity, error) {
	status, err := valueobject.ParseExampleEntityStatus(row.Status)
	if err != nil {
		return nil, fmt.Errorf("example_entity.map_row: %w", err)
	}

	var deletedAt *time.Time
	if row.DeletedAt != nil {
		t := *row.DeletedAt
		deletedAt = &t
	}

	return entity.ExampleEntityFromExisting(
		valueobject.ExampleEntityIDFromTrusted(row.ID),
		row.OwnerID,
		valueobject.ExampleEntityNameFromTrusted(row.Name),
		row.Url,
		status,
		row.CreatedAt,
		row.UpdatedAt,
		deletedAt,
	), nil
}

// mapPgError belongs in internal/infra/postgres/errors.go, shared by all
// repository implementations in this package. Shown here for completeness.
func mapPgError(op string, err error) error {
	if errors.Is(err, pgx.ErrNoRows) {
		return fmt.Errorf("%s: %w", op, repository.ErrNotFound)
	}

	var pgErr *pgconn.PgError
	if errors.As(err, &pgErr) {
		switch pgErr.Code {
		case "23505":
			return fmt.Errorf("%s: %s: %w", op, pgErr.ConstraintName, repository.ErrUniqueViolation)
		case "23503":
			return fmt.Errorf("%s: %s: %w", op, pgErr.ConstraintName, repository.ErrForeignKeyViolation)
		}
	}

	return fmt.Errorf("%s: %w", op, err)
}
