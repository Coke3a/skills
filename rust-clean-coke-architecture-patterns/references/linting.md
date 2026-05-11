# Linting

## Why Linting Matters

The Rust compiler catches many mistakes, but deeper analysis requires `cargo clippy`. Clippy checks for:
- Performance pitfalls
- Style issues
- Redundant code
- Potential bugs
- Non-idiomatic Rust

## Running Clippy

Standard invocation with strict warnings:

```shell
cargo clippy --all-targets --all-features --locked -- -D warnings
```

Flags explained:
- `--all-targets`: checks library, tests, benches, and examples.
- `--all-features`: enables all features to check all conditional code.
- `--locked`: requires `Cargo.lock` to be up-to-date.
- `-D warnings`: treats warnings as errors (CI-friendly).

Optional additions:
- `-W clippy::pedantic`: stricter lints with occasional false positives.
- `-W clippy::nursery`: experimental lints still under development.

Add this to your Makefile, Justfile, xtask, or CI pipeline.

## Important Clippy Lints

| Lint                  | Category   | Why It Matters                                              |
|-----------------------|------------|-------------------------------------------------------------|
| `redundant_clone`     | perf       | Detects unnecessary `.clone()` calls with performance impact |
| `needless_borrow`     | style      | Removes redundant `&` borrowing                              |
| `large_enum_variant`  | perf       | Warns when an enum variant is very large; suggests `Box`     |
| `needless_collect`    | nursery    | Prevents unnecessary iterator collection allocations         |
| `clone_on_copy`       | complexity | Catches `.clone()` on `Copy` types like `u32`, `bool`       |
| `unnecessary_wraps`   | pedantic   | Flags functions that always return `Some`/`Ok`               |
| `map_unwrap_or`       | style      | Simplifies nested `Option`/`Result` handling                 |
| `manual_ok_or`        | style      | Suggests `.ok_or_else` instead of `match`                    |

## Fixing vs Silencing Warnings

**Never** silence a warning with `#[allow(...)]` unless you truly understand the cause and have a documented reason.

Prefer `#[expect(clippy::lint_name)]` over `#[allow(clippy::lint_name)]`. The `expect` attribute warns you when the lint no longer applies, preventing stale suppression.

Always add a justification comment:

```rust
// Faster matching is preferred over size efficiency for this hot path
#[expect(clippy::large_enum_variant)]
enum Message {
    Code(u8),
    Content([u8; 1024]),
}
```

### Handling False Positives

1. Try to refactor the code so the warning goes away naturally.
2. If not possible, locally suppress with `#[expect(clippy::lint_name)]` plus a comment.
3. Avoid global overrides unless it is a fundamental crate-level concern.

## Lint Configuration in Cargo.toml

### Package-level

```toml
[lints.rust]
future-incompatible = "warn"
nonstandard_style = "deny"

[lints.clippy]
all = { level = "deny", priority = 10 }
redundant_clone = { level = "deny", priority = 9 }
pedantic = { level = "warn", priority = 3 }
```

### Workspace-level

```toml
[workspace.lints.rust]
future-incompatible = "warn"
nonstandard_style = "deny"

[workspace.lints.clippy]
all = { level = "deny", priority = 10 }
redundant_clone = { level = "deny", priority = 9 }
pedantic = { level = "warn", priority = 3 }
```

When two lints conflict, the higher priority value wins.

## CI Integration

- Run `cargo clippy --all-targets --all-features --locked -- -D warnings` in CI.
- Fail the build on any clippy warning.
- Combine with `cargo fmt --check` for formatting enforcement.
- Run `cargo test --doc` separately if using `cargo nextest` (nextest skips doc tests).
- Consider adding clippy checks to pre-commit hooks or xtask workflows.
