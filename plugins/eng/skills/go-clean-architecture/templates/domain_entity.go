// Template: replace ExampleEntity, ExampleEntityID, ExampleEntityName,
// ExampleEntityStatus, and field names with project-specific names.
// Target location: internal/domain/entity/{entity}.go
// Replace "app" in imports with the project's module path.
// Do not import Fiber, pgx, sqlc-generated code, handler DTOs, or infra types here.
package entity

import (
	"time"

	"github.com/google/uuid"

	"app/internal/domain"
	"app/internal/domain/valueobject"
)

type ExampleEntity struct {
	id        valueobject.ExampleEntityID
	ownerID   uuid.UUID
	name      valueobject.ExampleEntityName
	url       string
	status    valueobject.ExampleEntityStatus
	createdAt time.Time
	updatedAt time.Time
	deletedAt *time.Time
}

// NewExampleEntity creates a fresh entity with default state.
func NewExampleEntity(
	ownerID uuid.UUID,
	name valueobject.ExampleEntityName,
	url string,
) *ExampleEntity {
	now := time.Now().UTC()

	return &ExampleEntity{
		id:        valueobject.NewExampleEntityID(),
		ownerID:   ownerID,
		name:      name,
		url:       url,
		status:    valueobject.ExampleEntityStatusActive,
		createdAt: now,
		updatedAt: now,
	}
}

// ExampleEntityFromExisting reconstructs an entity from persisted data.
// Only infra repository implementations call this; it performs no validation
// because the values were validated when first created.
func ExampleEntityFromExisting(
	id valueobject.ExampleEntityID,
	ownerID uuid.UUID,
	name valueobject.ExampleEntityName,
	url string,
	status valueobject.ExampleEntityStatus,
	createdAt time.Time,
	updatedAt time.Time,
	deletedAt *time.Time,
) *ExampleEntity {
	return &ExampleEntity{
		id:        id,
		ownerID:   ownerID,
		name:      name,
		url:       url,
		status:    status,
		createdAt: createdAt,
		updatedAt: updatedAt,
		deletedAt: deletedAt,
	}
}

func (e *ExampleEntity) ID() valueobject.ExampleEntityID {
	return e.id
}

func (e *ExampleEntity) OwnerID() uuid.UUID {
	return e.ownerID
}

func (e *ExampleEntity) Name() valueobject.ExampleEntityName {
	return e.name
}

func (e *ExampleEntity) URL() string {
	return e.url
}

func (e *ExampleEntity) Status() valueobject.ExampleEntityStatus {
	return e.status
}

func (e *ExampleEntity) CreatedAt() time.Time {
	return e.createdAt
}

func (e *ExampleEntity) UpdatedAt() time.Time {
	return e.updatedAt
}

func (e *ExampleEntity) DeletedAt() *time.Time {
	return e.deletedAt
}

func (e *ExampleEntity) IsDeleted() bool {
	return e.deletedAt != nil
}

func (e *ExampleEntity) Rename(name valueobject.ExampleEntityName) error {
	if err := e.ensureNotDeleted(); err != nil {
		return err
	}

	e.name = name
	e.updatedAt = time.Now().UTC()
	return nil
}

func (e *ExampleEntity) ChangeStatus(status valueobject.ExampleEntityStatus) error {
	if err := e.ensureNotDeleted(); err != nil {
		return err
	}

	e.status = status
	e.updatedAt = time.Now().UTC()
	return nil
}

func (e *ExampleEntity) SoftDelete() error {
	if err := e.ensureNotDeleted(); err != nil {
		return err
	}

	now := time.Now().UTC()
	e.status = valueobject.ExampleEntityStatusInactive
	e.updatedAt = now
	e.deletedAt = &now
	return nil
}

func (e *ExampleEntity) ensureNotDeleted() error {
	if e.IsDeleted() {
		return &domain.InvariantError{Reason: "example entity is deleted"}
	}

	return nil
}
