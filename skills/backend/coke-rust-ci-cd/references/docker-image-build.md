# Docker Image Build

- Use a multi-stage Dockerfile for Rust services.
- Build in a Rust builder image and copy only the final binary to runtime.
- Add `.dockerignore` to exclude `target`, `.git`, local env files, and secrets.
- Prefer a non-root runtime user when practical.
- Keep the runtime image small.
- Do not bake secrets into the image.
- Tag images with the git SHA.
- Use `latest` only as a convenience tag for `main`, never as the only deployment identity.
- Prefer immutable image digest or SHA tag for deployment.
- Use Docker Buildx for build and push workflows.
- Authenticate to GHCR with `packages: write` only when publishing.
- Capture image digest output for deploy summaries.
- Read runtime configuration from environment variables or platform config.
