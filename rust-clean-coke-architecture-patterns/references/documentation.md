# Documentation

## Comments vs Docs

| Purpose        | `// comment`                        | `/// doc` / `//! crate doc`                              |
|----------------|-------------------------------------|----------------------------------------------------------|
| Describe Why   | Yes -- explains tricky reasoning    | No                                                       |
| Describe API   | No                                  | Yes -- public interfaces, usage, errors, panics          |
| Maintainability| Often becomes stale                 | Tied to code, appears in `cargo doc`, can run test cases  |
| Visibility     | Local development only              | Exported to users and tools                              |

## When to Use Comments

Use `//` when something cannot be expressed clearly in code:
- **Safety guarantees**: `// SAFETY: ptr is guaranteed non-null by caller`
- **Workarounds or optimizations**: explain why the non-obvious approach was chosen.
- **Platform-specific behavior**: when `#[cfg(..)]` alone is not sufficient context.
- **Links to design docs or ADRs**: `// CONTEXT: [ADR-12](link/to/adr-12)`
- **Assumptions or gotchas** that are not obvious from the code.

Name your comments with a prefix: `// SAFETY:`, `// CONTEXT:`, `// WORKAROUND:`.

## Don't Write Living Comments

Comments that restate code are dangerous:
- They rot -- nobody compiles comments.
- They mislead -- readers assume they are true without questioning.
- They go stale unless maintained with the code.
- They clutter code with unnecessary noise.

If something deserves to live beyond a PR, put it in an ADR, design document, doc comments with examples, or tests.

## Replacing Comments with Better Code

Instead of commenting blocks, extract named helper functions:

```rust
// BAD: comments restating each step
fn save_user(&self) -> Result<(), MyError> {
    // check if authenticated
    if self.is_authenticated() {
        // serialize user data
        let data = serde_json::to_string(self)?;
        // write to file
        std::fs::write(self.path(), data)?;
    }
}

// GOOD: code explains itself
fn save_auth_user(&self) -> Result<PathBuf, MyError> {
    if self.is_authenticated() {
        let serialized = serde_json::to_string(self)?;
        std::fs::write(self.path(), serialized)?;
        Ok(self.path())
    } else {
        Err(MyError::UserNotAuthenticated)
    }
}
```

## Doc Comment Structure

Use `///` for all public functions, structs, traits, enums, and constants.

```rust
/// Loads a [`User`] profile from disk.
///
/// # Examples
///
/// ```rust
/// # use my_crate::load_user;
/// let user = load_user(Path::new("profile.json")).unwrap();
/// ```
///
/// # Errors
/// - Returns [`MyError::FileNotFound`] if the file is missing.
/// - Returns [`MyError::InvalidJson`] if content is invalid JSON.
///
/// # Panics
/// Panics if the path contains non-UTF8 characters.
pub fn load_user(path: &Path) -> Result<User, MyError> { /* ... */ }
```

Sections to include as relevant: summary, `# Examples`, `# Errors`, `# Panics`, `# Safety`.

## Module-Level Documentation

Use `//!` at the top of `lib.rs` or `mod.rs` to document module/crate purpose:

```rust
//! Chess engine implementation.
//!
//! Handles board state, move generation, and check detection.
//!
//! # Example
//! ```
//! let board = chess::engine::Board::default();
//! assert!(board.is_valid());
//! ```
```

## Documentation Lints

| Lint                    | Description                                                    |
|-------------------------|----------------------------------------------------------------|
| `missing_docs`          | Warns on public items without documentation                    |
| `broken_intra_doc_links`| Detects broken internal doc links (catches renames)            |
| `empty_docs`            | Prevents empty doc comments that bypass `missing_docs`         |
| `missing_errors_doc`    | Warns when `Result`-returning functions lack `# Errors`        |
| `missing_panics_doc`    | Warns when panicking functions lack `# Panics`                 |
| `missing_safety_doc`    | Warns when unsafe functions lack `# Safety`                    |

Enable in top-level modules: `#![deny(missing_docs)]`.

## TODO Format

Never leave orphaned TODOs. File an issue first, then reference it:

```rust
// TODO(#42): Remove workaround after upstream bugfix
```

This makes TODOs trackable, actionable, and visible to everyone.

## When to Document vs Self-Document

- **Document**: public APIs, error conditions, panic conditions, safety invariants, non-obvious design choices.
- **Self-document through code**: use descriptive names, extract helper functions, use types to encode constraints, write tests that demonstrate behavior.
- Use `cargo doc --open` to check output regularly.
