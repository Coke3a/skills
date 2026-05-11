# Testing

## Tests as Living Documentation

Tests are the first place people look to understand how code works. Clear, targeted tests are often more helpful than reading the function body itself. Together, tests form living documentation that stays accurate because the compiler enforces it.

## Descriptive Test Naming

Use the pattern: `{function}_should_{expected}_when_{condition}`.

```rust
#[cfg(test)]
mod test {
    mod process {
        #[test]
        fn should_return_blob_when_larger_than_b() {
            let a = setup_a();
            let b = Some(2);
            let expected = MyExpectedStruct { /* ... */ };

            let result = process(a, b).unwrap();

            assert_eq!(result, expected);
        }
    }
}
```

When running `cargo test`, output reads as: `process::should_return_blob_when_larger_than_b`.

Use nested `mod` blocks to group tests for the same function. IDEs can run entire modules at once.

## One Behavior Per Test

Each test should describe one thing the unit does. This makes failures immediately understandable.

```rust
// BAD: testing two behaviors
fn test_parser() {
    assert!(Thing::parse("abcd").is_ok());
    assert!(Thing::parse("ABCD").is_err());
}

// GOOD: separate tests, separate behaviors
mod thing_parser {
    #[test]
    fn should_accept_lowercase_letters() {
        assert!(Thing::parse("abcd").is_ok(), "Parse error: {:?}", Thing::parse("abcd").unwrap_err());
    }

    #[test]
    fn should_reject_uppercase_letters() {
        assert!(Thing::parse("ABCD").is_err());
    }
}
```

Use `rstest` for parameterized cases when testing the same behavior across inputs:

```rust
#[rstest]
#[case::single("a")]
#[case::first_letter("ab")]
fn accepts_strings_with_a(#[case] input: &str) {
    assert!(the_function(input).is_ok());
}
```

## Doc Tests

Use `///` doc comments with code examples on public APIs. These run with `cargo test` (but not `cargo nextest run` -- run `cargo test --doc` separately).

```rust
/// Adds two numbers together.
///
/// # Examples
///
/// ```rust
/// # use my_crate::add;
/// assert_eq!(add(2, 3), 5);
/// ```
pub fn add(a: i32, b: i32) -> i32 { a + b }
```

- Hide boilerplate with `#` prefix lines.
- Duplication between doc tests and unit tests is fine.
- Doc test attributes: `should_panic`, `no_run`, `compile_fail`, `ignore`.

## Unit vs Integration vs Doc Tests

### Unit Tests
- Same module as tested code, inside `#[cfg(test)] mod test { ... }`.
- Can access private functions and `pub(crate)` items.
- Focus on implementation details, edge cases, and error paths.
- Use `#[ignore = "reason"]` for incomplete tests, `#[should_panic]` for expected panics.

### Integration Tests
- Live under `tests/` directory, external to the library.
- Can only test the public API.
- Test that multiple units work together correctly.
- Use `testcontainers` for external dependencies.

### Doc Tests
- Show happy-path public API usage.
- Serve as both documentation and correctness checks.

## Snapshot Testing with cargo insta

Use snapshot testing when correctness is structural (generated code, serialized data, CLI output, rendered HTML).

```toml
insta = { version = "1.42.2", features = ["yaml"] }
```

```rust
#[test]
fn test_split_words() {
    let words = split_words("hello from the other side");
    insta::assert_yaml_snapshot!(words);
}
```

Run `cargo insta test` then `cargo insta review`.

Best practices:
- Use named snapshots for meaningful file names.
- Keep snapshots small -- snapshot subsections, not huge objects.
- Use redactions for unstable fields (timestamps, UUIDs).
- Commit snapshots to git and review changes carefully.
- Prefer `assert_eq!` for simple/primitive values over snapshots.

## Assert Macros

- `assert!(expr)` for booleans, `assert_eq!(left, right)` for equality.
- Always add formatted failure messages: `assert_eq!(result, expected, "Diff: {}", result.diff(expected))`.
- For Ok scenarios, print the Err case on failure.
- Use `matches!` with `assert!` for pattern matching without extracting values:
  ```rust
  assert!(matches!(error, MyError::BadInput(_)), "Expected BadInput, found {error}");
  ```
- Use `#[should_panic]` only when panic is the desired behavior; prefer `Result` otherwise.
- Consider `pretty_assertions` crate for colorful diffs.

## Architecture-Specific Testing

### Usecase Tests with Mock Repos
Usecases depend on repository traits. In tests, create mock implementations:

```rust
#[cfg(test)]
mod test {
    struct MockUserRepo { /* preconfigured returns */ }
    impl UserRepository for MockUserRepo { /* ... */ }

    #[test]
    fn should_return_user_when_found() {
        let repo = MockUserRepo { /* setup */ };
        let usecase = GetUserUsecase::new(Arc::new(repo));
        let result = usecase.execute(user_id).unwrap();
        assert_eq!(result.name, "expected_name");
    }
}
```

### Domain Unit Tests
Test value objects and entities directly. They have no dependencies:

```rust
#[test]
fn email_should_reject_missing_at_sign() {
    assert!(Email::try_from("invalid").is_err());
}

#[test]
fn money_should_add_same_currency() {
    let a = Money::new(100, Currency::USD);
    let b = Money::new(50, Currency::USD);
    assert_eq!(a + b, Money::new(150, Currency::USD));
}
```

## Testing Error Paths

Always test error paths explicitly. Use `.unwrap_err()` and match against expected errors:

```rust
#[test]
fn should_return_invalid_input_when_both_missing() {
    let result = process(None, None).unwrap_err();
    assert_eq!(result, MyError::InvalidInput);
}
```

Test that errors propagate correctly through the usecase layer and map to the right domain errors.
