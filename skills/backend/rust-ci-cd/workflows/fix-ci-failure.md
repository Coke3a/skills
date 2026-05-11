# Fix CI/CD Failure

1. Classify failure
- [ ] YAML syntax.
- [ ] Permissions.
- [ ] Missing secret or env.
- [ ] rustfmt.
- [ ] clippy.
- [ ] Tests.
- [ ] Build.
- [ ] Docker.
- [ ] Registry auth.
- [ ] Migration.
- [ ] Deploy.
- [ ] Smoke test.
- [ ] Concurrency or cancellation.

2. Find root cause
- [ ] Read logs.
- [ ] Identify first failing step.
- [ ] Distinguish CI config failure from code failure.
- [ ] Distinguish environment failure from test failure.
- [ ] Do not hide real code failures with CI changes.

3. Fix
- [ ] Apply smallest safe fix.
- [ ] Do not weaken checks unless explicitly justified.
- [ ] Do not skip tests without reason.
- [ ] Do not broaden permissions unless required.
- [ ] Update docs or summary if secrets or vars changed.

4. Verify
- [ ] Run local equivalent command if possible.
- [ ] Report what cannot be verified locally.
- [ ] Do not fabricate remote CI success.
