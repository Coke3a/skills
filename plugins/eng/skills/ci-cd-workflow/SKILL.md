---
name: ci-cd-workflow
description: Design, implement, review, or repair language-neutral CI/CD pipelines from change validation through artifact promotion, deployment verification, and rollback. Use for pipeline triggers, checks, caching, artifacts, environments, migrations, permissions, secrets, concurrency, or failed automation. Adapt commands and providers to the repository; do not use for application feature work or provisioning an entire platform.
---

# CI/CD Workflow

Build a trustworthy path from source change to a verified release. The repository determines language-specific commands and the delivery platform; this skill determines pipeline shape, safety, and evidence.

## Discover the existing contract

Inspect manifests, lockfiles, task runners, scripts, container files, current workflows, branch rules, environments, deployment docs, and infrastructure configuration. Prefer canonical project commands over commands invented in workflow YAML. Preserve the repository's chosen CI provider and deployment target unless the user asks to change them.

Identify:

- events and branches that should trigger validation or release;
- required format, lint, type, test, security, build, and packaging checks;
- the immutable artifact promoted between environments;
- environment approvals, credentials, migrations, smoke tests, and rollback mechanism.

## Pipeline model

1. Validate changes on pull requests with no production credentials.
2. Build once from an identified commit and record immutable artifact metadata.
3. Publish the artifact only after required checks pass.
4. Promote the same artifact through environments rather than rebuilding it.
5. Serialize deployments per environment and make supersession behavior explicit.
6. Run migrations with compatibility and recovery considered before application rollout.
7. Verify the deployed system with a bounded smoke or health check.
8. Stop and roll back or forward-fix according to the documented failure policy.

Not every repository needs every stage. Keep the pipeline proportional to release risk and avoid decorative jobs that add latency without catching a plausible failure.

## Security and reliability

Use least-privilege job permissions, pin or otherwise govern third-party actions, isolate untrusted pull-request code from secrets, prefer short-lived identity federation over long-lived credentials, mask sensitive output, and scope secrets to environments. Never fabricate successful command, deploy, migration, or smoke-test results.

Cache dependencies only when keys include the relevant lockfile and toolchain inputs; caches are accelerators, not release artifacts. Set timeouts and concurrency deliberately. Make retries bounded and reserve them for transient, idempotent operations.

Read [references/pipeline-decisions.md](references/pipeline-decisions.md) for migration, artifact, deployment, and rollback decisions.

## Boundaries

Use `coke-eng:ops-docker-vm-deploy` when the deployment target is a Dockerized application on a user-managed VM over SSH. This skill may orchestrate that deployment but does not replace its host, exposure, or rollback details. Provisioning Kubernetes, cloud accounts, networks, or a complete observability platform requires separately authorized work.

## Definition of done

Report changed workflows and triggers, canonical commands, artifact identity, permissions, secret sources, concurrency, environment gates, migration behavior, post-deploy verification, rollback path, validations actually run, and unresolved operational assumptions.
