# Repository + ORM

## ORM stack
- Diesel 2.3 + diesel-async with deadpool connection pooling.
- `PgPool` type alias in `infra::db::postgres_connection` wraps `deadpool::Pool<AsyncPgConnection>`.
- Pool max size: 10 connections.
- Repositories hold `Arc<PgPool>`.

## Repository port pattern
- Define the trait in `src/domain/repositories/{entity}_repository.rs` using `async_trait`.
- Implement the trait in `src/infra/db/repositories/{entity}_postgres.rs`.
- Not-found is `Option<T>` for queries, `RepoError::NotFound` for updates/deletes that expect existing rows.
- All methods return `Result<T, RepoError>`.

## Row conversion pattern
Repositories use two internal structs:

1. **Read row** (`EntityRow`): `#[derive(Queryable, Selectable)]` with `into_entity()` method.
2. **Insert row** (`NewEntityRow`): `#[derive(Insertable)]` with `from_entity(&entity)` associated function using borrowed fields.

```rust
#[derive(Queryable, Selectable)]
#[diesel(table_name = endpoints)]
struct EndpointRow {
    id: Uuid,
    user_id: Uuid,
    name: String,
    // ...
}

impl EndpointRow {
    fn into_entity(self) -> Endpoint {
        Endpoint::from_existing(
            EndpointId::from_uuid(self.id),
            self.user_id,
            EndpointName::from_trusted(self.name),  // from_trusted skips validation
            // ...
        )
    }
}

#[derive(Insertable)]
#[diesel(table_name = endpoints)]
struct NewEndpointRow<'a> {
    id: &'a Uuid,
    user_id: &'a Uuid,
    name: &'a str,
    // ...
}

impl<'a> NewEndpointRow<'a> {
    fn from_entity(entity: &'a Endpoint) -> Self {
        Self {
            id: entity.id().as_uuid(),
            user_id: entity.user_id(),
            name: entity.name().as_str(),
            // ...
        }
    }
}
```

## Repository struct pattern
```rust
pub struct EndpointPostgres {
    pool: Arc<PgPool>,
}

impl EndpointPostgres {
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
.map_err(|e| map_diesel_error("endpoint.find_by_id", e))?;
```

Operation names follow the pattern `entity.operation` (e.g., `"endpoint.create"`, `"endpoint.find_by_id"`).

## Common query patterns

### Find by ID (returns Option)
```rust
let result = endpoints::table
    .find(id.as_uuid())
    .first::<EndpointRow>(&mut conn)
    .await
    .optional()
    .map_err(|e| map_diesel_error("endpoint.find_by_id", e))?;

Ok(result.map(|row| row.into_entity()))
```

### Insert
```rust
let new_row = NewEndpointRow::from_entity(endpoint);

diesel::insert_into(endpoints::table)
    .values(&new_row)
    .execute(&mut conn)
    .await
    .map_err(|e| map_diesel_error("endpoint.create", e))?;
```

### Update with not-found check
```rust
let rows_affected = diesel::update(endpoints::table.find(endpoint.id().as_uuid()))
    .set(( /* fields */ ))
    .execute(&mut conn)
    .await
    .map_err(|e| map_diesel_error("endpoint.update", e))?;

if rows_affected == 0 {
    return Err(RepoError::NotFound(format!("Endpoint {} not found", endpoint.id())));
}
```

### Optimistic locking (WHERE guards)
```rust
// update_if_active: only update if status is still active
let rows_affected = diesel::update(
    sessions::table
        .find(session.id().as_uuid())
        .filter(sessions::status.eq_any(["connected", "reconnecting"]))
    )
    .set(( /* fields */ ))
    .execute(&mut conn)
    .await
    .map_err(|e| map_diesel_error("session.update_if_active", e))?;

Ok(rows_affected > 0)  // Returns bool indicating if update succeeded
```

### Transaction pattern
```rust
use diesel_async::{AsyncConnection, scoped_futures::ScopedFutureExt};

conn.transaction::<_, diesel::result::Error, _>(|conn| {
    async move {
        let count = endpoints::table
            .filter(endpoints::user_id.eq(user_id))
            .count()
            .get_result::<i64>(conn)
            .await?;

        if count >= max_endpoints {
            return Ok(false);
        }

        diesel::insert_into(endpoints::table)
            .values(&new_row)
            .execute(conn)
            .await?;

        Ok(true)
    }
    .scope_boxed()
})
.await
.map_err(|e| map_diesel_error("endpoint.create_if_under_limit", e))
```

### UPSERT pattern (ON CONFLICT)
```rust
diesel::insert_into(rate_limits::table)
    .values(&new_row)
    .on_conflict((rate_limits::endpoint_id, rate_limits::hour_bucket))
    .do_update()
    .set(rate_limits::event_count.eq(rate_limits::event_count + 1))
    .returning(RateLimitRow::as_returning())
    .get_result(&mut conn)
    .await
    .map_err(|e| map_diesel_error("rate_limit.increment_or_create", e))?;
```

## Creating repos in handlers
Repositories are instantiated in handlers from `AppState`:
```rust
let endpoint_repo: Arc<dyn EndpointRepository> =
    Arc::new(EndpointPostgres::new(Arc::clone(&state.db_pool)));
let usecase = CreateEndpointUseCase::new(endpoint_repo, subscription_repo);
```
