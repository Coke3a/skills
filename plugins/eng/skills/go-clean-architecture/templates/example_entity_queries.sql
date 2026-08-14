-- Template: sqlc query file. Target location: db/queries/{entity}.sql
-- Run `sqlc generate` after every change; names become methods on gen.Queries.
-- :one → single row, :many → slice, :exec → no result, :execrows → affected count.

-- name: CreateExampleEntity :exec
INSERT INTO example_entities (id, owner_id, name, url, status, created_at, updated_at)
VALUES ($1, $2, $3, $4, $5, $6, $7);

-- name: GetExampleEntityByID :one
SELECT id, owner_id, name, url, status, created_at, updated_at, deleted_at
FROM example_entities
WHERE id = $1 AND deleted_at IS NULL;

-- name: ListExampleEntitiesByOwner :many
SELECT id, owner_id, name, url, status, created_at, updated_at, deleted_at
FROM example_entities
WHERE owner_id = $1 AND deleted_at IS NULL
ORDER BY created_at DESC
LIMIT $2 OFFSET $3;

-- name: UpdateExampleEntity :execrows
UPDATE example_entities
SET name = $2, url = $3, status = $4, updated_at = $5
WHERE id = $1 AND deleted_at IS NULL;

-- name: SoftDeleteExampleEntity :execrows
UPDATE example_entities
SET status = $2, updated_at = $3, deleted_at = $3
WHERE id = $1 AND deleted_at IS NULL;

-- name: DeleteExampleEntity :execrows
DELETE FROM example_entities
WHERE id = $1;
