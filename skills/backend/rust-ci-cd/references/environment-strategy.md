# Environment Strategy

- Use `staging` for automatic or semi-automatic deploys from `main`.
- Use `production` for manual, release-tag, or protected-environment deploys.
- Store secrets separately per environment.
- Require approvals for production where the repository supports protected environments.
- Use branch restrictions for deployment environments when useful.
- Use `workflow_dispatch` to redeploy selected artifacts.
- Use separate concurrency groups such as `staging-deploy` and `production-deploy`.
- Preserve deployment history through job summaries, deployment records, or platform logs.
- Keep production deploy gates explicit.
