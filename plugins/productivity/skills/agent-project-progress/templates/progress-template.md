# PROGRESS.md Template

Use this structure unless `project/PROGRESS.md` already has a project-specific format. Preserve useful existing content and update it in place rather than overwriting.

```markdown
# Project Progress

## Current Snapshot

Status:
Current task:
Current objective:
Last updated:

## What Was Done

## Current State

## Verification Done

## Next Steps

## Open Questions

## Risks / Blockers

## Changed Areas
```

## `Changed Areas` grouping

Group work by repository or project area, using the actual discovered names. Do not invent fixed names like `backend/` or `frontend/`.

Example:

```markdown
## Changed Areas

- `api/`: Updated authentication handler and request validation.
- `web/`: Added login form error state.
- `project/tests/`: Added cross-repo smoke test notes.
```

Possible project areas:

- application, service, worker, mobile, or admin repository
- shared package
- `project/docs/`
- `project/tests/`
- `project/scripts/`
- database or migration area
- deployment or CI area

Do not assume any repository exists until inspected.
