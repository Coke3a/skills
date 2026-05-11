# Rust Quality Checklist

- Modules and files use snake_case.
- Types and traits use UpperCamelCase.
- Functions and methods use snake_case.
- Constants use SCREAMING_SNAKE_CASE.
- Getter methods usually use `field_name()`, not `get_field_name()`.
- Conversion methods follow `as_`, `to_`, and `into_` conventions.
- Acronyms follow Rust style, such as `Uuid`, `Http`, and `Api`, unless the project
  differs.
- Prefer borrowing when ownership is not needed.
- Avoid unnecessary `clone()`, `to_string()`, `format!()`, `collect()`, and allocation
  on hot paths.
- Avoid returning references that make callers fight lifetimes without benefit.
- Prefer owned DTO and boundary types when it simplifies lifetimes.
- Avoid `Arc<Mutex<T>>` by default when immutable `Arc<T>`, ownership transfer, or
  message passing is clearer.
- Do not use `unwrap()` or `expect()` in production unless explicitly justified and
  safe.
- Use `Result` and `?` for recoverable failures.
- Use typed errors such as `thiserror` when that is the project pattern.
- Preserve useful context without leaking sensitive details.
- Avoid blocking work in async code.
- Do not hold locks or borrows across `.await`.
- Avoid hidden panics from user input.
- Avoid unbounded concurrency without backpressure.
- Treat Clippy as an automated guardrail.
- Treat rustfmt as a formatting guardrail.
- Add comments/doc comments when public behavior needs explanation.
