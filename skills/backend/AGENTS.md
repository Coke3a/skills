# AGENTS.md

## Project

Backend project using Clean Architecture. Two stacks are supported, each with its own skill set:

| Stack | Detect by    | Architecture skill                              |
| ----- | ------------ | ----------------------------------------------- |
| Rust  | `Cargo.toml` | `coke-rust-clean-architecture` (Diesel)         |
| Go    | `go.mod`     | `coke-go-clean-architecture` (Fiber v3 + sqlc + pgx) |

Identify the stack before choosing any skill, and do not mix the two sets. The layer boundaries are
deliberately identical — `handlers → usecases → domain`, `infra → domain interfaces` — but the
idioms, error flow, repository pattern, and verification commands are not interchangeable, and
advice from the wrong stack reads plausible while being wrong. If a repository contains both, work
out which service the change belongs to before starting.

## Available Backend Skills

### Rust

- **coke-rust-clean-architecture**: layer structure, naming, error flow, repository traits, Diesel
  repository implementations, handler/usecase/domain boundaries.

- **coke-tdd-feature-workflow**: new behavior, bug fixes, acceptance criteria, test scope selection,
  red/green/refactor, Rust test file placement.

- **coke-rust-code-review**: before finishing a feature or PR — correctness, architecture, tests,
  Rust quality, error handling, async/concurrency, security, verification readiness.

- **coke-rust-ci-cd**: GitHub Actions, Rust CI checks, Docker/image builds, deployment workflows,
  migrations, smoke tests, rollback, permissions, secrets, OIDC.

- **coke-rust-performance-optimization**: only when there is a performance goal, hot path,
  benchmark/profile need, suspected bottleneck, or performance regression.

### Go

- **coke-go-clean-architecture**: layer structure, naming, error flow, repository interfaces, sqlc
  query files and pgx repository implementations, Fiber v3 handlers and central error handling,
  project/feature directory layout, background workers with an owned lifecycle.

- **coke-go-tdd-feature-workflow**: new behavior, bug fixes, acceptance criteria, test level
  selection, red/green/refactor, Go test placement. Runs in one phase only — the design overview is
  settled (endpoints, request/response shapes, schema) and the deep behavior is not — and hard-stops
  outside it. Covers domain, usecase, and handler levels; repository integration is deliberately out
  of scope.

- **coke-go-performance-optimization**: only when there is a performance goal, hot path,
  benchmark/profile need, suspected bottleneck, or performance regression.

### Any stack

- **coke-docker-vm-deploy**: building a Docker image in GitHub Actions and shipping it to a Linux VM
  over SSH. Language-agnostic, so it is the deploy path for Go as well as Rust.

Gaps to be aware of rather than paper over: Go has no dedicated code-review or CI skill.
`coke-rust-code-review` and `coke-rust-ci-cd` are Rust-specific by construction and must not be
applied to Go code — use the architecture checklist inside `coke-go-clean-architecture` instead, and
say plainly that the review was a checklist pass rather than a skill-driven review. Note also that
`coke-tdd-feature-workflow` is the Rust TDD skill and `coke-go-tdd-feature-workflow` is the Go one;
the names are one word apart and the conventions are not interchangeable.

## Required Commands Before Finishing Code Changes

Rust:

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features
```

Go:

```bash
go build ./...
go vet ./...
go test -race ./...
golangci-lint run   # when the project has a golangci-lint config
```

`-race` is not optional in Go: the whole point of the worker and pool patterns in the architecture
skill is concurrent lifetime safety, and a data race is invisible without it.

## Development Rules

Both stacks:

- Keep handlers thin.
- Keep business logic in domain/usecases.
- Keep repository abstractions in domain.
- Do not leak DTOs into domain.
- Add or update tests for changed behavior.
- Do not create all test levels upfront.
- Do not bypass architecture boundaries for performance.
- Do not fabricate command results.

Rust:

- Keep repository traits in domain and Diesel implementations in infra.
- Do not leak Diesel rows outside infra.

Go:

- Keep repository interfaces in domain and pgx/sqlc implementations in `internal/infra/postgres`.
- Do not leak sqlc-generated row or params structs outside `internal/infra/postgres`.
- Domain must not import Fiber, pgx, or sqlc-generated packages.
- Values from `c.Params()`, `c.Query()`, `c.Body()`, `c.Get()` are only valid until the handler
  returns — fasthttp reuses the buffer. Bind into a DTO, or copy, before storing or passing to a
  goroutine.
- Every goroutine needs a stop signal and an exit wait owned by the thing that started it. No
  fire-and-forget `go func()`.

## Feature Workflow

Rust:

1. Use `coke-tdd-feature-workflow` to define acceptance criteria and the smallest useful first test.
2. Use `coke-rust-clean-architecture` to place code in the correct layers.
3. Implement one behavior at a time.
4. Run tests.
5. Use `coke-rust-code-review` before finishing.

Go:

1. Use `coke-go-tdd-feature-workflow` to run its entry gate, derive acceptance criteria, and drive
   one behavior at a time through red/green/refactor. The repository interface is discovered by the
   usecase test rather than written from the schema, which is the same reasoning as step 2 below,
   enforced by a compiler instead of by discipline.
2. Use `coke-go-clean-architecture` for layer, naming, and error-taxonomy decisions, and for the
   parts the TDD loop does not cover. Its `workflows/scaffold-feature.md` fixes the order (domain
   value objects and entity → ports → usecase → migration and sqlc queries → postgres repository →
   handler and router → wiring), which matters because each step compiles against the one before it.
   Deciding what the usecase needs before writing SQL is what keeps the schema serving the domain
   rather than the reverse.
3. Run the required commands above.
4. Verify repository persistence against a real database — the TDD loop deliberately excludes sqlc
   and pgx behavior, so nothing else covers it.
5. Walk the architecture checklist in `references/architecture.md` before calling it done.

## Performance Workflow

Same shape for both stacks; the skill differs (`coke-rust-performance-optimization` /
`coke-go-performance-optimization`).

1. Define the performance goal, including which metric — p99 and throughput are not the same target.
2. Measure a baseline under the workload that actually produces the symptom. If it only hurts under
   load or extra cores do not help, that baseline must be concurrent.
3. Identify the hot path with a profile, not a guess.
4. Make the smallest safe change.
5. Preserve architecture boundaries.
6. Re-measure the metric that was complained about, and check the ones that were not targeted for
   regressions.
7. Rust only: use `coke-rust-code-review` before finishing.

## CI/CD Workflow

- Rust: `coke-rust-ci-cd` for CI checks, build, deploy, smoke test, rollback, and hardening.
- Go: no CI skill yet — write the workflow directly from the required commands above.
- Either stack, deploying a container to a VM you own: `coke-docker-vm-deploy`.
