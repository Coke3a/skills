# Test File Placement

Recommended project convention:

```text
src/
  domain/
    value_objects/
      validated/
        example_entity_name.rs
        example_entity_name_test.rs
        mod.rs

    entities/
      example_entity.rs
      example_entity_test.rs
      mod.rs

  usecases/
    example_entities/
      create_example_entity.rs
      create_example_entity_test.rs
      mod.rs

tests/
  common/
    mod.rs

  repositories/
    example_entity_postgres_test.rs

  api/
    example_entities_api_test.rs
```

Rules:

- Domain and usecase unit tests may live beside the production module as `*_test.rs`.
- Every `*_test.rs` file under `src/` must be declared from the parent module with
  `#[cfg(test)]`.
- Repository and API integration tests live under `tests/`.
- Shared integration test helpers live under `tests/common/mod.rs`.
- Do not rely on the `*_test.rs` suffix alone for Rust source-level tests.
- Do not put DB/API integration tests in `src/`.
- Do not create `src/tests/`.
- Do not create `test_process/`.

Examples:

```rust
pub mod example_entity_name;

#[cfg(test)]
mod example_entity_name_test;
```

```rust
pub mod create_example_entity;

#[cfg(test)]
mod create_example_entity_test;
```
