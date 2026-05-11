# Optional background task architecture variant

Background job workflow is not owned by this skill. Use this note only when a periodic task must
preserve the same Clean Architecture dependency direction.

## Dependency direction

```text
handler/spawner -> usecase -> domain repository trait
infra repository -> domain repository trait
```

## Minimal architecture checklist

- [ ] Keep scheduling, intervals, cancellation, and task spawning outside domain
- [ ] Put periodic orchestration in a usecase
- [ ] Define needed repository methods as domain traits
- [ ] Implement repository methods in infra
- [ ] Keep Diesel rows private to infra
- [ ] Keep the task spawner free of business rules
- [ ] Use the same error flow as request/response usecases when errors cross layers

## Final commands

- [ ] `cargo fmt --all -- --check`
- [ ] `cargo clippy --all-targets --all-features -- -D warnings`
- [ ] `cargo test --all-features`
