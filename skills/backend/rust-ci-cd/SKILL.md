---
name: rust-ci-cd
description: Use when creating, reviewing, or fixing CI/CD for Rust backend projects. Covers GitHub Actions Rust checks, cargo fmt/clippy/test/build, caching, Docker image builds, staging/production deployment workflows, migrations, smoke tests, rollback guidance, concurrency, least-privilege permissions, secrets, and OIDC. Pair with rust-clean-coke-architecture-patterns, tdd-feature-workflow, and rust-code-review.
---

# Rust CI/CD

## Purpose

Create and repair CI/CD pipelines for Rust backend services. Focus on reliable Rust CI,
safe deployment automation, repeatable artifacts, environment separation, deployment
smoke tests, rollback readiness, and GitHub Actions security.

## When to Use

- Setting up CI for a Rust project.
- Adding cargo fmt, clippy, test, and release build checks.
- Creating or reviewing GitHub Actions workflows.
- Adding Docker image build and push workflows.
- Deploying to staging or production.
- Adding database migration steps.
- Adding smoke tests and rollback guidance.
- Hardening GitHub Actions permissions, secrets, and OIDC.
- Fixing failed CI/CD workflows.
- Reviewing deployment pipeline structure.

## When Not to Use

- Designing Rust clean architecture.
- Writing feature code.
- Choosing TDD test cases.
- Performing final Rust implementation code review.
- Benchmarking or load testing.
- Designing observability platforms.
- Provisioning cloud-provider-specific infrastructure unless explicitly requested.

## Core Rules

- CI must run on pull requests.
- Main branch must be protected by CI before deployment.
- Use default Rust checks in this order: fmt, clippy, test, build.
- Deploy from a CI artifact or image, not a local machine.
- Prefer the same artifact for staging and production.
- Keep secrets out of PR workflows.
- Use least-privilege permissions.
- Use concurrency to prevent duplicate deploys.
- Run smoke tests after deploy.
- Document rollback.
- Never fabricate command results.

## Default Rust Commands

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features
cargo build --release
```

Inspect `Cargo.toml` before choosing workspace-specific flags. Use existing Makefile,
justfile, or script commands when the repository already defines canonical local checks.

## Resource Guide

- Use `workflows/setup-rust-ci.md` for pull request and main-branch Rust CI.
- Use `workflows/setup-docker-build.md` for Dockerfile and image publishing setup.
- Use `workflows/setup-deploy-pipeline.md` for staging and production deployment.
- Use `workflows/setup-db-migration-pipeline.md` for database migration integration.
- Use `workflows/fix-ci-failure.md` to diagnose and repair failing CI/CD.
- Use `workflows/harden-github-actions.md` for permissions, secrets, OIDC, and shell
  hardening.
- Use `references/` for detailed guidance only when relevant.
- Use `templates/` as starting points, then adapt to the repository.

## Companion Skills

- `rust-clean-coke-architecture-patterns` owns architecture, layers, naming, error flow,
  and repository patterns.
- `tdd-feature-workflow` owns test design and the red/green/refactor workflow.
- `rust-code-review` owns final code review, quality review, security review, and
  async/concurrency review.
- `rust-ci-cd` owns automation that verifies and ships the code produced by those
  skills.

## Final Response Format

Summarize:

- Workflows created or changed.
- Triggers.
- Rust checks.
- Deployment environments.
- Required secrets and vars.
- Permissions used.
- Concurrency behavior.
- Artifact or image strategy.
- Migration behavior.
- Smoke test behavior.
- Rollback method.
- Commands or validations run.
- Risks or follow-up.
