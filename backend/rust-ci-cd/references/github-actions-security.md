# GitHub Actions Security

- Set explicit least-privilege `permissions`.
- Prefer job-level permissions when jobs need different scopes.
- Use `GITHUB_TOKEN` for repository-scoped operations when sufficient.
- Use OIDC with `id-token: write` for cloud authentication when possible.
- Store staging and production secrets in GitHub Environments.
- Avoid `pull_request_target`; if required, do not checkout untrusted code with secrets.
- Treat secret masking as best-effort, not a safe logging strategy.
- Pin third-party actions to full commit SHA when high security is required.
- Prefer trusted first-party actions when tag pinning is acceptable.
- Quote shell variables and avoid interpolating untrusted event fields into shell.
- Do not print secrets.
- Do not upload artifacts that contain secrets, env files, credentials, or tokens.
- Never expose production secrets to PR workflows or forks.
