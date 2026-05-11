# Rust CI Checks

Default order:

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features
cargo build --release
```

Workspace variants:

- Use `cargo fmt --all -- --check` for workspace formatting.
- Prefer `cargo clippy --workspace --all-targets --all-features -- -D warnings` only when the repository is a workspace and uses that pattern.
- Prefer `cargo test --workspace --all-features` only when appropriate for the workspace.
- Do not invent workspace flags for a single-crate project.

Integration test support:

- Add service containers only when tests require them.
- Use test-only environment variables.
- Never use production secrets in tests.
- Make service health checks explicit.

Optional checks:

- Add `cargo audit` or `cargo deny` only if the project chooses security scanning.
- Add coverage only when requested; coverage is not mandatory for this skill.
