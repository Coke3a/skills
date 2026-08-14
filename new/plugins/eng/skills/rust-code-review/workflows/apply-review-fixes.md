# Apply Review Fixes

1. Select findings

- [ ] Identify findings to fix.
- [ ] Do not fix unrelated code.
- [ ] Do not introduce new architecture patterns.
- [ ] Do not optimize unrelated hot paths without evidence.

2. Apply fixes

- [ ] Fix blockers first.
- [ ] Fix high severity findings.
- [ ] Keep changes small.
- [ ] Preserve behavior unless the finding is about incorrect behavior.
- [ ] Update tests if behavior changes.
- [ ] Do not weaken tests to make them pass.
- [ ] Do not add broad concurrency abstractions unless needed for the finding.

3. Verify

- [ ] Run narrow tests.
- [ ] Run cargo fmt/clippy/test if possible.
- [ ] Report commands run.

4. Summarize

- [ ] Findings fixed.
- [ ] Findings left open.
- [ ] Risks.
- [ ] Commands.
