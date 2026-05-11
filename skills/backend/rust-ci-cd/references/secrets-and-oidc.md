# Secrets And OIDC

- Store secrets in GitHub Secrets or GitHub Environments.
- Use environment vars for non-sensitive configuration.
- Use secrets for credentials, tokens, private keys, and database URLs.
- Prefer OIDC for cloud authentication instead of long-lived cloud keys.
- Avoid personal access tokens when `GITHUB_TOKEN` or OIDC is enough.
- If a PAT is required, scope it narrowly and store it as a repository or environment
  secret.
- Never echo secrets.
- Do not write secrets into artifacts, logs, Docker images, or generated files.
- Do not expose production secrets to PR workflows.
