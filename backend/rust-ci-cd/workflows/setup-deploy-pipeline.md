# Setup Deploy Pipeline

1. Choose deployment strategy
- [ ] Identify provider or platform.
- [ ] Identify deploy command or script.
- [ ] Identify staging and production environments.
- [ ] Identify secrets and vars.
- [ ] Identify artifact or image source.
- [ ] Identify rollback method.

2. Staging deploy
- [ ] Trigger after main branch CI or `workflow_dispatch`.
- [ ] Use staging environment.
- [ ] Use staging secrets only.
- [ ] Deploy known artifact or image.
- [ ] Run migrations if configured.
- [ ] Run smoke test.
- [ ] Publish deployment summary.

3. Production deploy
- [ ] Prefer `workflow_dispatch`, release tag, or protected environment approval.
- [ ] Use production environment.
- [ ] Use production secrets only.
- [ ] Deploy known-good artifact or image.
- [ ] Use concurrency to prevent overlapping production deploys.
- [ ] Run migrations if configured.
- [ ] Run smoke test.
- [ ] Publish rollback instructions.

4. Verify
- [ ] Do not expose production secrets to PRs.
- [ ] Do not deploy mutable `latest` without recording SHA or digest.
- [ ] Do not run destructive migrations without approval.
- [ ] Do not fabricate deploy success.
