# Repository And Diesel Reference

Use this reference for repository traits and Diesel repository implementations.

## Repository port pattern

- Define traits in `src/domain/repositories/{entity}_repository.rs`.
- Keep repository traits as persistence ports only; do not put Diesel, schema, row structs, or pool
  types in domain.
- Implement traits in `src/infra/db/repositories/{entity}_postgres.rs`.
- Use `src/domain/services/` for external-service ports that are not persistence concerns.
- Use `async_trait` for async trait methods.
- Return `Result<T, RepoError>`.
- `find_by_*` methods return `Result<Option<T>, RepoError>`.
- Updates/deletes that expect an existing row return `RepoError::NotFound` when no row is affected.
- Keep `mod.rs` files declaration-only with only `pub mod ...;`.

```rust
#[async_trait]
pub trait ExampleRepository: Send + Sync {
    async fn create(&self, entity: &ExampleEntity) -> Result<(), RepoError>;
    async fn find_by_id(&self, id: &ExampleEntityId) -> Result<Option<ExampleEntity>, RepoError>;
    async fn update(&self, entity: &ExampleEntity) -> Result<(), RepoError>;
    async fn delete(&self, id: &ExampleEntityId) -> Result<(), RepoError>;
}
```

## Diesel implementation pattern

- Name the implementation `{Entity}Postgres`.
- Store `Arc<PgPool>`.
- Get a connection inside each method.
- Use Diesel query builder only.
- Use centralized `map_diesel_error()` and `map_pool_error()`.
- Keep rows private to infra.
- Return domain entities or value objects from repository methods, never row structs or DTOs.

```rust
pub struct ExamplePostgres {
    pool: Arc<PgPool>,
}

impl ExamplePostgres {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }
}
```

## Row conversion pattern

Use one row struct for reads and one row struct for inserts.

```rust
#[derive(Queryable, Selectable)]
#[diesel(table_name = example_entities)]
struct ExampleEntityRow {
    id: Uuid,
    owner_id: Uuid,
    column_text: String,
    column_url: String,
    status: String,
}

impl ExampleEntityRow {
    fn into_entity(self) -> ExampleEntity {
        ExampleEntity::from_existing(
            ExampleEntityId::from_uuid(self.id),
            self.owner_id,
            ExampleEntityName::from_trusted(self.column_text),
            self.column_url,
            ExampleEntityStatus::from_trusted(self.status),
        )
    }
}

#[derive(Insertable)]
#[diesel(table_name = example_entities)]
struct NewExampleEntityRow<'a> {
    id: &'a Uuid,
    owner_id: &'a Uuid,
    column_text: &'a str,
    column_url: &'a str,
    status: &'a str,
}

impl<'a> NewExampleEntityRow<'a> {
    fn from_entity(entity: &'a ExampleEntity) -> Self {
        Self {
            id: entity.id().as_uuid(),
            owner_id: entity.owner_id(),
            column_text: entity.column_text().as_str(),
            column_url: entity.column_url(),
            status: entity.status().as_str(),
        }
    }
}
```

## Error mapping

```rust
let mut conn = self.pool.get().await.map_err(map_pool_error)?;

example_entities::table
    .find(id.as_uuid())
    .first::<ExampleEntityRow>(&mut conn)
    .await
    .optional()
    .map_err(|err| map_diesel_error("example_entity.find_by_id", err))?;
```

Operation names should follow `entity.operation`, such as:

- `example_entity.create`
- `example_entity.find_by_id`
- `example_entity.update`
- `example_entity.delete`

## Query patterns

### Create

```rust
let new_row = NewExampleEntityRow::from_entity(entity);

diesel::insert_into(example_entities::table)
    .values(&new_row)
    .execute(&mut conn)
    .await
    .map_err(|err| map_diesel_error("example_entity.create", err))?;
```

### Find by ID

```rust
let row = example_entities::table
    .find(id.as_uuid())
    .first::<ExampleEntityRow>(&mut conn)
    .await
    .optional()
    .map_err(|err| map_diesel_error("example_entity.find_by_id", err))?;

Ok(row.map(ExampleEntityRow::into_entity))
```

### Update

```rust
let rows_affected = diesel::update(example_entities::table.find(entity.id().as_uuid()))
    .set((
        example_entities::column_text.eq(entity.column_text().as_str()),
        example_entities::column_url.eq(entity.column_url()),
        example_entities::status.eq(entity.status().as_str()),
    ))
    .execute(&mut conn)
    .await
    .map_err(|err| map_diesel_error("example_entity.update", err))?;

if rows_affected == 0 {
    return Err(RepoError::NotFound(format!("example entity {} not found", entity.id())));
}
```

### Delete

```rust
let rows_affected = diesel::delete(example_entities::table.find(id.as_uuid()))
    .execute(&mut conn)
    .await
    .map_err(|err| map_diesel_error("example_entity.delete", err))?;

if rows_affected == 0 {
    return Err(RepoError::NotFound(format!("example entity {id} not found")));
}
```

## Optional transaction shape

Use transactions only when one usecase requires multiple writes to commit atomically.

```rust
use diesel_async::{scoped_futures::ScopedFutureExt, AsyncConnection};

conn.transaction::<_, diesel::result::Error, _>(|conn| {
    async move {
        diesel::insert_into(example_entities::table)
            .values(&new_row)
            .execute(conn)
            .await?;

        Ok(())
    }
    .scope_boxed()
})
.await
.map_err(|err| map_diesel_error("example_entity.transactional_create", err))?;
```
