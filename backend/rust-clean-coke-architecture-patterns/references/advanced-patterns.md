# Advanced Rust Patterns Are Out Of Scope

This Rust Clean Architecture skill does not define advanced Rust patterns such as type-
state APIs, generic dispatch strategy, pointer taxonomy, or optimization-oriented
abstractions.

The architecture default is:

- Use `Arc<dyn RepositoryTrait>` in usecases.
- Keep repository traits in domain.
- Keep repository implementations in infra.

Use a dedicated Rust patterns or performance skill for broader guidance.
