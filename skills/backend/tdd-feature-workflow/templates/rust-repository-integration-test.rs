// Template destination:
// tests/repositories/<entity>_postgres_test.rs
//
// Use this only when persistence behavior, Diesel mapping,
// database constraints, transactions, or DB error mapping are introduced or changed.

#[path = "../common/mod.rs"]
mod common;

use common::setup_test_database;
use example_app::domain::entities::example_entity::ExampleEntity;
use example_app::domain::entities::example_entity::ExampleEntityId;
use example_app::domain::value_objects::example_entity_name::ExampleEntityName;
use example_app::infrastructure::repositories::errors::RepoError;
use example_app::infrastructure::repositories::example_postgres::ExamplePostgres;

#[tokio::test]
async fn inserts_and_finds_entity() {
    let db = setup_test_database().await; // project-specific placeholder
    db.run_migrations().await; // project-specific placeholder
    let repo = ExamplePostgres::new(db.pool());
    let entity = ExampleEntity::new(ExampleEntityName::new("Valid name").unwrap());

    repo.insert(&entity).await.unwrap();
    let found = repo.find_by_id(entity.id()).await.unwrap();

    assert_eq!(found.id(), entity.id());
    assert_eq!(found.name(), entity.name());
}

#[tokio::test]
async fn updates_existing_entity() {
    let db = setup_test_database().await; // project-specific placeholder
    db.run_migrations().await; // project-specific placeholder
    let repo = ExamplePostgres::new(db.pool());
    let mut entity = ExampleEntity::new(ExampleEntityName::new("Original name").unwrap());
    repo.insert(&entity).await.unwrap();

    entity.rename(ExampleEntityName::new("Updated name").unwrap()).unwrap();
    repo.update(&entity).await.unwrap();
    let found = repo.find_by_id(entity.id()).await.unwrap();

    assert_eq!(found.name().as_str(), "Updated name");
}

#[tokio::test]
async fn deletes_existing_entity() {
    let db = setup_test_database().await; // project-specific placeholder
    db.run_migrations().await; // project-specific placeholder
    let repo = ExamplePostgres::new(db.pool());
    let entity = ExampleEntity::new(ExampleEntityName::new("Valid name").unwrap());
    repo.insert(&entity).await.unwrap();

    repo.delete(entity.id()).await.unwrap();
    let result = repo.find_by_id(entity.id()).await;

    assert!(matches!(result, Err(RepoError::NotFound)));
}

#[tokio::test]
async fn maps_row_to_entity() {
    let db = setup_test_database().await; // project-specific placeholder
    db.run_migrations().await; // project-specific placeholder
    db.insert_example_row("fixed-id", "Persisted name").await; // project-specific placeholder
    let repo = ExamplePostgres::new(db.pool());

    let found = repo.find_by_external_id("fixed-id").await.unwrap();

    assert_eq!(found.name().as_str(), "Persisted name");
}

#[tokio::test]
async fn returns_not_found_for_missing_entity() {
    let db = setup_test_database().await; // project-specific placeholder
    db.run_migrations().await; // project-specific placeholder
    let repo = ExamplePostgres::new(db.pool());

    let result = repo.find_by_id(ExampleEntityId::new_for_test()).await;

    assert!(matches!(result, Err(RepoError::NotFound)));
}

#[tokio::test]
async fn returns_conflict_for_unique_constraint_violation() {
    let db = setup_test_database().await; // project-specific placeholder
    db.run_migrations().await; // project-specific placeholder
    let repo = ExamplePostgres::new(db.pool());
    let first = ExampleEntity::new(ExampleEntityName::new("Unique name").unwrap());
    let duplicate = ExampleEntity::new(ExampleEntityName::new("Unique name").unwrap());

    repo.insert(&first).await.unwrap();
    let result = repo.insert(&duplicate).await;

    assert!(matches!(result, Err(RepoError::Conflict(_))));
}
