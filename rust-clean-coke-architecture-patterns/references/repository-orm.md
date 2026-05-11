# Repository + ORM

## ORM stack
- Diesel 2.2+ with `diesel-async` and `deadpool` connection pooling.
- `PgPool` type alias wraps `deadpool::Pool<AsyncPgConnection>`.
- Pool max size configurable (typically 10 connections).
- Repositories hold `Arc<PgPool>`.
- Key rule: NO raw SQL -- always use the Diesel query builder.

## Repository port pattern
- Define trait in `src/domain/repositories/{entity}_repository.rs` using `async_trait`.
- Implement trait in `src/infra/db/repositories/{entity}_postgres.rs`.
- Not-found is `Option<T>` for queries, `RepoError::NotFound` for updates/deletes expecting existing rows.
- All methods return `Result<T, RepoError>`.

## Row conversion pattern

Two internal structs per entity:

**Read row** (`EntityRow`): `#[derive(Queryable, Selectable)]` with `into_entity()` method.
**Insert row** (`NewEntityRow`): `#[derive(Insertable)]` with `from_entity(&entity)` using borrowed fields.

```rust
#[derive(Queryable, Selectable)]
#[diesel(table_name = items)]
struct ItemRow {
    id: Uuid,
    user_id: Uuid,
    name: String,
}

impl ItemRow {
    fn into_entity(self) -> Item {
        Item::from_existing(
            ItemId::from_uuid(self.id),
            self.user_id,
            ItemName::from_trusted(self.name), // from_trusted skips validation
        )
    }
}

#[derive(Insertable)]
#[diesel(table_name = items)]
struct NewItemRow<'a> {
    id: &'a Uuid,
    user_id: &'a Uuid,
    name: &'a str,
}

impl<'a> NewItemRow<'a> {
    fn from_entity(entity: &'a Item) -> Self {
        Self {
            id: entity.id().as_uuid(),
            user_id: entity.user_id(),
            name: entity.name().as_str(),
        }
    }
}
```

## Repository struct pattern
```rust
pub struct ItemPostgres {
    pool: Arc<PgPool>,
}

impl ItemPostgres {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }
}
```

## Error mapping pattern
All repositories use shared helpers from `error_mapping.rs`:
```rust
use super::error_mapping::{map_diesel_error, map_pool_error};

// Getting a connection:
let mut conn = self.pool.get().await.map_err(map_pool_error)?;

// Running a query:
.await
.map_err(|e| map_diesel_error("item.find_by_id", e))?;
```

Operation names follow the pattern `entity.operation` (e.g., `"item.create"`, `"item.find_by_id"`).

## Common query patterns

### Find by ID (returns Option)
```rust
let result = items::table
    .find(id.as_uuid())
    .first::<ItemRow>(&mut conn)
    .await
    .optional()
    .map_err(|e| map_diesel_error("item.find_by_id", e))?;

Ok(result.map(|row| row.into_entity()))
```

### Insert
```rust
let new_row = NewItemRow::from_entity(item);

diesel::insert_into(items::table)
    .values(&new_row)
    .execute(&mut conn)
    .await
    .map_err(|e| map_diesel_error("item.create", e))?;
```

### Update with not-found check
```rust
let rows_affected = diesel::update(items::table.find(item.id().as_uuid()))
    .set(( /* fields */ ))
    .execute(&mut conn)
    .await
    .map_err(|e| map_diesel_error("item.update", e))?;

if rows_affected == 0 {
    return Err(RepoError::NotFound(format!("Item {} not found", item.id())));
}
```

### Delete
```rust
let rows_affected = diesel::delete(items::table.find(id.as_uuid()))
    .execute(&mut conn)
    .await
    .map_err(|e| map_diesel_error("item.delete", e))?;

if rows_affected == 0 {
    return Err(RepoError::NotFound(format!("Item {} not found", id)));
}
```

### Count
```rust
let count = items::table
    .filter(items::user_id.eq(user_id))
    .count()
    .get_result::<i64>(&mut conn)
    .await
    .map_err(|e| map_diesel_error("item.count_by_user", e))?;
```

### Transaction
```rust
use diesel_async::{AsyncConnection, scoped_futures::ScopedFutureExt};

conn.transaction::<_, diesel::result::Error, _>(|conn| {
    async move {
        let count = items::table
            .filter(items::user_id.eq(user_id))
            .count()
            .get_result::<i64>(conn)
            .await?;

        if count >= max_items {
            return Ok(false);
        }

        diesel::insert_into(items::table)
            .values(&new_row)
            .execute(conn)
            .await?;

        Ok(true)
    }
    .scope_boxed()
})
.await
.map_err(|e| map_diesel_error("item.create_if_under_limit", e))
```

### Upsert (ON CONFLICT)
```rust
diesel::insert_into(counters::table)
    .values(&new_row)
    .on_conflict((counters::entity_id, counters::bucket))
    .do_update()
    .set(counters::count.eq(counters::count + 1))
    .returning(CounterRow::as_returning())
    .get_result(&mut conn)
    .await
    .map_err(|e| map_diesel_error("counter.increment_or_create", e))?;
```

### Optimistic locking (WHERE guards)
```rust
let rows_affected = diesel::update(
    sessions::table
        .find(session.id().as_uuid())
        .filter(sessions::status.eq_any(["active", "pending"]))
    )
    .set(( /* fields */ ))
    .execute(&mut conn)
    .await
    .map_err(|e| map_diesel_error("session.update_if_active", e))?;

Ok(rows_affected > 0)  // Returns bool indicating if update succeeded
```

## Creating repos in handlers
```rust
let item_repo: Arc<dyn ItemRepository> =
    Arc::new(ItemPostgres::new(Arc::clone(&state.db_pool)));
let usecase = CreateItemUseCase::new(item_repo);
```
