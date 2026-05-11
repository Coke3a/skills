# Broad Rust Idioms Are Out Of Scope

This Rust Clean Architecture skill does not define general Rust idiom guidance.

Use this skill only for idioms directly required by the architecture pattern:

- `Arc<dyn RepositoryTrait>` for usecase dependency injection.
- Private entity fields with getters.
- Value object constructors returning `Result<Self, DomainError>`.
- `?` with `From` conversions across layer error types.
- Borrowed fields in Diesel insert rows.

Use a dedicated Rust idioms skill for broader language guidance.
