# Setup Rust CI

1. Inspect project

- [ ] Read `Cargo.toml`.
- [ ] Determine if project is single crate or workspace.
- [ ] Check existing CI workflows.
- [ ] Check existing Makefile, justfile, or scripts.
- [ ] Check required services for tests.
- [ ] Do not invent commands if project already has canonical commands.

2. Create CI workflow

- [ ] Add `.github/workflows/rust-ci.yml`.
- [ ] Trigger on `pull_request` and push to `main`.
- [ ] Set least-privilege permissions.
- [ ] Add concurrency for branch CI.
- [ ] Checkout code.
- [ ] Install or use Rust toolchain.
- [ ] Cache Cargo dependencies.
- [ ] Run `cargo fmt`.
- [ ] Run `cargo clippy`.
- [ ] Run `cargo test`.
- [ ] Run `cargo build --release`.

3. Handle services

- [ ] If repository tests require Postgres, Redis, or another service, add service containers only
      when needed.
- [ ] Use test-specific environment variables.
- [ ] Do not use production secrets in CI.
- [ ] Keep service health checks explicit.

4. Verify

- [ ] Validate workflow YAML.
- [ ] Report commands included.
- [ ] Do not fabricate GitHub Actions results.
- [ ] If possible, run equivalent local commands.
