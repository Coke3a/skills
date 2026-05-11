# Smoke Tests And Rollback

Smoke test candidates:

- `GET /healthz` returns 200.
- `GET /readyz` returns 200.
- Version endpoint reports the deployed commit.
- Ready endpoint confirms database connectivity.

Rules:

- Keep smoke tests fast.
- Do not mutate production data unless explicitly safe.
- Do not require broad secrets.
- Fail the deployment job when a critical smoke test fails.
- If smoke tests are skipped, explain why in the deployment summary.

Rollback:

- Prefer redeploying the previous known-good image digest.
- Use previous git SHA tag when digest is unavailable.
- Use platform-native rollback when available.
- Revert and redeploy only when artifact rollback is unavailable.
- Document migration rollback caveats when schema changes are not backward-compatible.
- Include deployed artifact, previous artifact, rollback command, migration status, and
  smoke test result in the deploy summary.
