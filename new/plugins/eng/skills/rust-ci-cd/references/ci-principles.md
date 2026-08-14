# CI Principles

- Provide fast feedback on every pull request.
- Keep `main` deployable by running CI before deployment.
- Use deterministic checks that match local developer commands.
- Fail early on formatting, lint, test, and build errors.
- Keep workflows small and focused.
- Do not deploy unreviewed PR code.
- Build once and deploy the produced artifact or image.
- Use the same deployment process for staging and production when possible.
- Keep environment-specific configuration outside the artifact.
- Run smoke tests after deployment.
- Keep rollback instructions available for production.
- Use least-privilege permissions and minimize secret exposure.
