# Harden GitHub Actions

1. Permissions
- [ ] Add explicit permissions.
- [ ] Use `contents: read` by default.
- [ ] Add `packages: write` only for package or image publish.
- [ ] Add `id-token: write` only for OIDC.
- [ ] Avoid `write-all`.

2. Secrets
- [ ] Keep secrets in GitHub Secrets or Environments.
- [ ] Use staging and production environment separation.
- [ ] Do not expose secrets to PR workflows.
- [ ] Avoid long-lived cloud credentials when OIDC is available.

3. Actions
- [ ] Prefer trusted actions.
- [ ] Pin third-party actions to SHA if the project requires high security.
- [ ] Avoid untrusted `curl | bash`.
- [ ] Avoid `pull_request_target` unless hardened.

4. Shell
- [ ] Quote variables.
- [ ] Use `set -euo pipefail` where practical.
- [ ] Avoid direct interpolation of untrusted event fields into shell.
- [ ] Avoid printing secrets.

5. Artifacts
- [ ] Do not upload secrets.
- [ ] Avoid storing env files with secrets.
- [ ] Limit retention when appropriate.
