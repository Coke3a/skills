# GitHub Actions Patterns

- Use `pull_request` for PR CI.
- Use push to `main` for main-branch validation and artifact publishing.
- Use `workflow_dispatch` for manual deploys, redeploys, and migrations.
- Use GitHub Environments for staging and production secrets.
- Use concurrency groups to prevent duplicate CI runs and overlapping deploys.
- Use `needs` to make deploy jobs depend on successful checks or build jobs.
- Use artifacts or immutable Docker image refs to pass build output between jobs.
- Cache Cargo dependencies with OS and `Cargo.lock` in the key.
- Use a matrix only when multiple Rust versions, feature sets, or OS targets are
  required.
- Add service containers only for integration tests that need them.
- Write deployment summaries to `$GITHUB_STEP_SUMMARY`.
- Avoid huge inline shell scripts; prefer version-controlled scripts for deployment
  logic.
