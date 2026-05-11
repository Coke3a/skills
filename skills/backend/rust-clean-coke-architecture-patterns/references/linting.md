# Linting Workflow Is Out Of Scope

This Rust Clean Architecture skill does not define lint configuration, lint policy, pre- commit
hooks, or CI enforcement.

The only lint command owned by this skill is the downstream final verification command:

```sh
cargo clippy --all-targets --all-features -- -D warnings
```

Use a dedicated CI/CD or linting skill for lint policy.
