# Rust Test Patterns

## Sibling source-level test file

```rust
pub mod example_entity_name;

#[cfg(test)]
mod example_entity_name_test;
```

```rust
use super::example_entity_name::*;
use crate::domain::errors::DomainError;

#[test]
fn rejects_empty_name() {
    let result = ExampleEntityName::new("");

    assert!(matches!(result, Err(DomainError::Validation(_))));
}
```

## Inline test module

Use inline tests only when tests are small and the project does not require sibling files.

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_valid_name() {
        let result = ExampleEntityName::new("Valid name");

        assert!(result.is_ok());
    }
}
```

## Integration test

Place integration tests under `tests/`.

```rust
// tests/repositories/example_entity_postgres_test.rs

#[tokio::test]
async fn finds_inserted_entity() {
    let db = setup_test_database().await;
    let repo = ExamplePostgres::new(db);

    let entity = ExampleEntity::new(ExampleEntityName::new("Valid name").unwrap());
    repo.insert(&entity).await.unwrap();

    let found = repo.find_by_id(entity.id()).await.unwrap();

    assert_eq!(found.id(), entity.id());
}
```

## Shared integration helpers

Prefer `tests/common/mod.rs`:

```rust
// tests/common/mod.rs

pub async fn setup_test_database() -> TestDatabase {
    todo!("project-specific test database setup")
}
```

## Async tests

Use `#[tokio::test]` when the project uses Tokio and the code under test is async.

## Hand-written fake repository

```rust
#[derive(Default)]
struct FakeExampleRepository {
    next_error: Option<RepoError>,
    saved: Vec<ExampleEntity>,
}

#[async_trait::async_trait]
impl ExampleRepository for FakeExampleRepository {
    async fn save(&mut self, entity: ExampleEntity) -> Result<(), RepoError> {
        if let Some(error) = self.next_error.take() {
            return Err(error);
        }
        self.saved.push(entity);
        Ok(())
    }
}
```

Prefer asserting outputs or resulting state over fake call counts. Assert a call count only when the
call itself is behavior.

## Assertions

Use `matches!` for typed error assertions:

```rust
assert!(matches!(result, Err(UsecaseError::Conflict(_))));
```

`unwrap` is acceptable in tests when it keeps setup readable. Avoid `unwrap` in production code.

## Test builders

Use builders when setup repeats and the builder makes the behavior easier to read:

```rust
let input = CreateExampleEntityInputBuilder::default()
    .name("Valid name")
    .build();
```
