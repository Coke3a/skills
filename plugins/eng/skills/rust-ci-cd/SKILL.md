---
name: rust-ci-cd
description: Guides CI/CD for Rust backend projects on GitHub Actions — fmt/clippy/test/build, caching, Docker image builds, staging/production deploys, database migrations, smoke tests, rollback, concurrency, least-privilege permissions, secrets, and OIDC. Use when setting up, hardening, or fixing CI/CD pipelines for a Rust service. Do not use for feature implementation, TDD, code review, or performance profiling.
---

# Rust CI/CD

## Use this when

- Setting up CI for a Rust project (fmt, clippy, test, release build).
- Creating or reviewing GitHub Actions workflows.
- Adding Docker image build and push workflows.
- Deploying to staging or production.
- Wiring database migrations into the pipeline.
- Adding smoke tests and rollback guidance.
- Hardening permissions, secrets, OIDC, or shell usage.
- Fixing a failed CI/CD workflow.

## Do not use this when

- Designing Rust clean architecture → use `coke-eng:rust-clean-architecture`.
- Writing feature code or designing tests → use `coke-eng:rust-tdd-feature-workflow`.
- Producing a final code review → use `coke-eng:rust-code-review`.
- Benchmarking, profiling, or load testing → use `coke-eng:rust-performance-optimization`.
- Designing observability platforms.
- Provisioning cloud-provider infrastructure unless explicitly requested.

## Core rules

- CI must run on pull requests; main branch must be protected by CI before deployment.
- Default Rust check order: `fmt`, `clippy`, `test`, `build`.
- Deploy from a CI artifact or image, not a local machine.
- Prefer the same artifact for staging and production.
- Keep secrets out of PR workflows; use least-privilege permissions.
- Use concurrency to prevent duplicate deploys.
- Run smoke tests after deploy and document rollback.
- Never fabricate command results.

## Workflow

1. Pick the workflow file matching the change (see Workflows below).
2. Inspect `Cargo.toml`, existing `Makefile`/`justfile`/scripts before choosing flags or commands.
3. Use templates as starting points; adapt to the repository.
4. Load reference files only when the relevant area needs deeper guidance.

## Default Rust commands

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features
cargo build --release
```

Use existing canonical commands (Makefile, justfile, or scripts) when the repository already defines them.

## Workflows

| Workflow                                          | Use for                                                  |
| ------------------------------------------------- | -------------------------------------------------------- |
| `workflows/setup-rust-ci.md`                      | Pull-request and main-branch Rust CI                     |
| `workflows/setup-docker-build.md`                 | Dockerfile and image publishing                          |
| `workflows/setup-deploy-pipeline.md`              | Staging and production deployment                        |
| `workflows/setup-db-migration-pipeline.md`        | Database migration integration                           |
| `workflows/fix-ci-failure.md`                     | Diagnosing and repairing failing CI/CD                   |
| `workflows/harden-github-actions.md`              | Permissions, secrets, OIDC, and shell hardening          |

## Load more detail

- CI principles and pipeline shape → `references/ci-principles.md`
- Standard Rust checks and flags → `references/rust-ci-checks.md`
- GitHub Actions patterns and caching → `references/github-actions-patterns.md`
- Permissions, secrets, OIDC, hardening → `references/github-actions-security.md`, `references/secrets-and-oidc.md`
- Docker image build patterns → `references/docker-image-build.md`
- Deployment automation, staging vs prod → `references/deployment-automation.md`
- Environment strategy → `references/environment-strategy.md`
- Migration safety in pipelines → `references/database-migrations.md`
- Smoke tests and rollback patterns → `references/smoke-tests-and-rollback.md`
- Common CI/CD smells → `references/ci-cd-smells.md`

## Templates

- `templates/deploy-summary.md` for the final deploy report.

## Related skills

- `coke-eng:rust-clean-architecture` — feature architecture and layer rules.
- `coke-eng:rust-tdd-feature-workflow` — tests that the CI runs.
- `coke-eng:rust-code-review` — final review of the code shipped through this pipeline.

## Definition of done

Summarise:

- Workflows created or changed and their triggers.
- Rust checks, deployment environments, required secrets/vars.
- Permissions used and concurrency behaviour.
- Artifact or image strategy and migration behaviour.
- Smoke test behaviour and rollback method.
- Commands or validations run, plus risks or follow-up.

<!-- update-probe: PROBE-1 -->
