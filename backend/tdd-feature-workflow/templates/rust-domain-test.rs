// Template destination:
// src/domain/.../<module_name>_test.rs
//
// Parent module must include:
// #[cfg(test)]
// mod <module_name>_test;
//
// Use this for pure domain behavior only.
// Do not use this template for database or HTTP tests.

use super::example_entity::*;
use super::example_entity_name::*;
use crate::domain::errors::DomainError;

#[test]
fn rejects_empty_name() {
    let result = ExampleEntityName::new("");

    assert!(matches!(result, Err(DomainError::Validation(_))));
}

#[test]
fn accepts_valid_name() {
    let result = ExampleEntityName::new("Valid name");

    assert!(result.is_ok());
    assert_eq!(result.unwrap().as_str(), "Valid name");
}

#[test]
fn creates_entity_with_valid_name() {
    let name = ExampleEntityName::new("Valid name").unwrap();

    let entity = ExampleEntity::new(name.clone());

    assert_eq!(entity.name(), &name);
}

#[test]
fn reconstructs_existing_entity_without_regenerating_id() {
    let id = ExampleEntityId::new_for_test();
    let name = ExampleEntityName::new("Valid name").unwrap();

    let entity = ExampleEntity::reconstruct(id, name.clone());

    assert_eq!(entity.id(), id);
    assert_eq!(entity.name(), &name);
}
