# Deployment Automation

- Deploy from a CI-produced artifact or image.
- Use the same deployment process for staging and production when possible.
- Keep deploy scripts in version control.
- Keep environment-specific configuration in environment vars, secrets, or platform config.
- Run migrations only when configured and safe.
- Stop deployment when required migrations fail.
- Run smoke tests after deploy.
- Document rollback for production.
- Write deployment summaries with environment, artifact, SHA, migration result, smoke test result,
  and rollback instruction.
- Do not deploy production from a developer machine.
- Do not rely on mutable artifact names without recording the exact SHA or digest.
