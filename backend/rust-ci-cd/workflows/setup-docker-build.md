# Setup Docker Build

1. Inspect project
- [ ] Identify binary crate name.
- [ ] Identify runtime port.
- [ ] Identify required runtime environment variables.
- [ ] Check existing Dockerfile.
- [ ] Check `.dockerignore`.

2. Create or update Dockerfile
- [ ] Use multi-stage build.
- [ ] Build Rust binary in builder stage.
- [ ] Copy final binary to runtime stage.
- [ ] Do not copy secrets.
- [ ] Use non-root user when practical.
- [ ] Keep image small.
- [ ] Expose only required port.

3. Create image workflow
- [ ] Trigger on push to `main` or `workflow_dispatch`.
- [ ] Run Rust CI checks before image publish or depend on CI workflow.
- [ ] Login to registry using least privilege.
- [ ] Build image.
- [ ] Tag image with git SHA.
- [ ] Optionally tag `latest` for `main`.
- [ ] Push image.
- [ ] Output image digest.

4. Verify
- [ ] Ensure image can run locally if possible.
- [ ] Ensure no secrets are baked into image.
- [ ] Ensure deployment uses immutable tag or digest.
