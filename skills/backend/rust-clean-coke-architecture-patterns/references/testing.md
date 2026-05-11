# Testing Reference Is Out Of Scope

This Rust Clean Architecture skill does not define a full testing strategy, test
pyramid, mock repository workflow, integration test strategy, UI test strategy, or TDD
cycle.

The only testing guidance owned by this skill is that downstream Rust projects using the
architecture pattern must pass:

```sh
cargo test --all-features
```

Use a dedicated testing or TDD skill for test design.
