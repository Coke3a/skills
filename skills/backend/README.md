# Backend Skills Playbook

Two stacks, two skill sets. The layer boundaries are deliberately identical —
`handlers → usecases → domain`, `infra → domain interfaces` — but the idioms, error flow, repository
pattern, and verification commands are not interchangeable. Advice from the wrong stack reads
plausible while being wrong.

**Identify the stack before picking a skill.**

| Stack | Detect by    | Architecture           | TDD                             |
| ----- | ------------ | ---------------------- | ------------------------------- |
| Rust  | `Cargo.toml` | Diesel                 | `coke-tdd-feature-workflow`     |
| Go    | `go.mod`     | Fiber v3 + sqlc + pgx  | `coke-go-tdd-feature-workflow`  |

The two TDD skills are one word apart in name and share nothing in convention. A repository holding
both services means working out which one the change belongs to first.

## Skills

### Rust

- **coke-rust-clean-architecture** — layer rules, file structure, naming, error flow, repository
  trait pattern, Diesel repository implementations, handler/usecase/domain boundaries.
- **coke-tdd-feature-workflow** — acceptance criteria, test scope selection, red/green/refactor,
  Rust test file placement. Covers domain, usecase, repository integration, and API levels.
- **coke-rust-code-review** — correctness, architecture, TDD test quality, Rust idioms, error
  handling, async/concurrency, security, review findings.
- **coke-rust-ci-cd** — GitHub Actions, Rust CI checks, Docker builds, deployment workflows,
  migrations, smoke tests, rollback, secrets, permissions, OIDC hardening.
- **coke-rust-performance-optimization** — measure-first optimization, benchmark/profile workflow,
  hot-path identification, async/Tokio performance, allocation/clone reduction, DB performance,
  before/after reporting.

### Go

- **coke-go-clean-architecture** — layer rules, naming, error flow, repository interfaces, sqlc
  query files and pgx implementations, Fiber v3 handlers and central error handling, project layout,
  background workers with an owned goroutine lifecycle.
- **coke-go-tdd-feature-workflow** — acceptance criteria, test level selection, red/green/refactor,
  Go test placement, hand-written fakes. **Gated:** runs only when the design overview is settled
  (endpoints, request/response shapes, schema) and the deep behavior is not. Covers domain, usecase,
  and handler levels; repository integration is deliberately out of scope.
- **coke-go-performance-optimization** — measure-first optimization, pprof workflow, allocation and
  escape analysis, GC tuning, goroutine/worker throughput, Fiber and database performance.

### Any stack

- **coke-docker-vm-deploy** (`skills/ops/`) — build a Docker image in GitHub Actions and ship it to a
  plain Linux VM over SSH. Language-agnostic, so it is the deploy path for both stacks.

## Gaps

State these plainly rather than substituting the Rust skill:

- **Go has no code-review skill.** Use the architecture checklist in
  `coke-go-clean-architecture/references/architecture.md` and say the review was a checklist pass,
  not a skill-driven review.
- **Go has no CI skill.** Write the workflow directly from the verification commands below.
- **Go TDD does not cover repository integration.** sqlc query correctness, row→entity mapping,
  constraints, and transactions need a real database and a separate verification pass.

`coke-rust-code-review` and `coke-rust-ci-cd` are Rust-specific by construction. Do not apply them to
Go code.

## Recommended Workflow

### Project start

**Rust**

1. `coke-rust-ci-cd` to set up CI.
2. `coke-rust-clean-architecture` to create the architecture skeleton.
3. `coke-rust-code-review` before the first commit.

**Go**

1. `coke-go-clean-architecture` to create the skeleton — `workflows/scaffold-feature.md` fixes the
   order because each step compiles against the one before it.
2. Write CI by hand from the verification commands below.

### Feature development

**Rust**

1. `coke-tdd-feature-workflow` to define behavior and test order.
2. `coke-rust-clean-architecture` for layer, file, error, and repository structure.
3. Implement one behavior at a time.
4. `coke-rust-code-review` before finishing.

**Go**

1. `coke-go-tdd-feature-workflow` — it runs its entry gate first and stops if the design overview is
   incomplete. Then one criterion at a time through red/green/refactor.
2. `coke-go-clean-architecture` for layer, naming, and error-taxonomy decisions, and for the parts
   the TDD loop leaves — sqlc queries, the pgx implementation, wiring.
3. Verify persistence against a real database. Nothing in the TDD loop covers it.
4. Walk the architecture checklist before calling it done.

The Go order matters: writing the usecase test first is what discovers the repository interface, so
the schema ends up serving the domain rather than the reverse.

### Bug fix

1. TDD skill for the stack — regression test first.
2. Fix the smallest correct behavior.
3. Rust: `coke-rust-code-review`. Go: architecture checklist.

### Performance

Same shape for both stacks; only the skill differs.

1. Define the goal, including which metric — p99 and throughput are not the same target.
2. Measure a baseline under the workload that actually produces the symptom. If it only hurts under
   load, that baseline must be concurrent.
3. Find the hot path with a profile, not a guess.
4. Make the smallest safe change.
5. Preserve architecture boundaries.
6. Re-measure the metric that was complained about, and check the ones that were not targeted.

### CI/CD

- Rust: `coke-rust-ci-cd`.
- Go: write the workflow from the verification commands below.
- Either stack, deploying a container to a VM you own: `coke-docker-vm-deploy`.

## Common Prompts

### New project

> Rust: "Use coke-rust-ci-cd to set up initial CI, then use coke-rust-clean-architecture to create a
> minimal Rust backend clean architecture skeleton. Before finishing, use coke-rust-code-review."

> Go: "Use coke-go-clean-architecture to create a minimal Go backend clean architecture skeleton with
> Fiber v3, sqlc, and pgx."

### New feature

> Rust: "Use coke-tdd-feature-workflow together with coke-rust-clean-architecture to implement this
> feature one behavior at a time. Do not create all test levels upfront. Before finishing, use
> coke-rust-code-review."

> Go: "The design overview is approved — endpoints, payloads, and schema are in `<path>`. Use
> coke-go-tdd-feature-workflow with coke-go-clean-architecture to design the deeper flow through
> tests, one criterion at a time."

### Code review

> Rust: "Use coke-rust-code-review to review the current changes for correctness, architecture
> boundaries, TDD test quality, Rust idioms, error handling, async/concurrency safety, security, and
> verification readiness."

> Go: "Walk the architecture checklist in coke-go-clean-architecture over the current changes, and
> tell me plainly that this is a checklist pass rather than a skill-driven review."

### Performance

> "Use coke-<stack>-performance-optimization with coke-<stack>-clean-architecture. Measure first,
> identify the hot path, optimize the smallest safe change, preserve architecture boundaries, then
> re-measure."

### CI/CD

> Rust: "Use coke-rust-ci-cd to create or fix GitHub Actions workflows for Rust CI, Docker build,
> staging/production deploy, migrations, smoke tests, rollback, and secrets/permissions."

> Either: "Use coke-docker-vm-deploy to build the image in Actions and ship it to the VM over SSH."

## Required Verification

**Rust**

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features
```

CI/CD projects also:

```bash
cargo build --release
```

**Go**

```bash
go build ./...
go vet ./...
go test -race ./...
golangci-lint run   # when the project has a golangci-lint config
```

`-race` is not optional in Go. The whole point of the worker and pool patterns in the architecture
skill is concurrent lifetime safety, and a data race is invisible without it.

If sqlc queries or migrations changed, `sqlc generate` was run and the generated diff is committed.

## Notes

- Do not use every skill for every task. Use skills by phase.
- Architecture skill is not TDD. TDD skill is not CI/CD.
- Code review is a quality gate, not a design phase.
- Performance optimization is on-demand and measurement-first.
- Do not bypass architecture boundaries for performance.
- Do not create all test levels upfront.
- Do not fabricate command results.
