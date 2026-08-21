# Pipeline decisions

## Validation

Run fast deterministic checks early and expensive checks only after cheap failures are excluded. Derive the exact commands from repository scripts and manifests. Split jobs when isolation or parallelism improves feedback; keep them together when repeated setup costs more than it saves.

## Artifact promotion

An artifact should be immutable and traceable to source revision, build inputs, and workflow run. Promote the same digest or package through environments. Do not use a mutable tag as the only release identity.

## Database migrations

Determine whether the application and schema changes are backward compatible during rollout. Decide who runs migrations, whether concurrent attempts are locked, what happens on partial failure, and whether recovery is rollback or a forward migration. Do not assume an application rollback can reverse a destructive schema change.

## Deployment verification

A process being alive is weaker evidence than a representative request succeeding. Use a bounded check that exercises the smallest critical path without mutating valuable data. Record enough diagnostics to distinguish rollout, configuration, dependency, and application failures.

## Rollback

Define the last known-good artifact, trigger or command, decision owner, and data compatibility constraint before relying on rollback. Automatic rollback is appropriate only when the signal is trustworthy and the operation is safe and idempotent.
